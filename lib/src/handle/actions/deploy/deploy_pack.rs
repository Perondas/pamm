use crate::handle::addons::ResolveAddons;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::files::name_consts::MP_MISSIONS_DIR_NAME;
use crate::io::fs::util::clean_path::canonicalize_and_clean_path;
use crate::io::fs::util::path_checks::is_file;
use crate::io::fs::util::symlink::create_or_recreate_symlink;
use anyhow::{Context, anyhow, ensure};
use log::{debug, warn};
use run_script::ScriptOptions;
use std::path::{Path, PathBuf};
use tokio::fs;

impl ServerRepoHandle {
    pub async fn deploy_pack(&self, pack_name: &str) -> anyhow::Result<()> {
        let server_dir = self.server_config.server_dir.as_ref().ok_or(anyhow!(
            "Server directory is not set in the server configuration.",
        ))?;

        let pamm_dir = server_dir.join("pamm");

        fs::create_dir_all(&pamm_dir)
            .await
            .context(anyhow!("Failed to create directory at {:?}", pamm_dir))?;

        let symlink_path = pamm_dir.join(&self.get_config().name);
        create_or_recreate_symlink(self.get_repo_path(), &symlink_path).context(anyhow!(
            "Failed to create symlink for server from {:?} to {:?}",
            self.get_repo_path(),
            symlink_path
        ))?;

        debug!(
            "Created symlink for server from {:?} to {:?}",
            self.get_repo_path(),
            symlink_path
        );

        let resolved_addons = self.resolve_addons(pack_name).await?;

        debug!(
            "Resolved addons for pack '{}': {:?}",
            pack_name, resolved_addons
        );

        let mut keys = Vec::new();
        for addon in &resolved_addons {
            keys.extend(get_path_to_keys(addon).await?);
        }

        self.symlink_keys(keys.into_iter(), server_dir).await?;

        debug!(
            "Symlinked keys for pack '{}' to server directory at {:?}",
            pack_name, server_dir
        );

        let addon_paths = resolved_addons
            .iter()
            .map(|rel| {
                let p = Path::new("pamm").join(&self.get_config().name).join(rel);
                p.to_str()
                    .map(|s| s.to_string())
                    .context(anyhow!("Failed to convert path to string: {:?}", p))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let mod_launch_param = format!("\"-mod={}\"", addon_paths.join(";"));

        debug!(
            "Deploying pack '{}' with mod launch parameter: {}",
            pack_name, mod_launch_param
        );

        self.symlink_missions(pack_name, server_dir).await?;

        self.process_script_templates(pack_name, &mod_launch_param)
            .await?;

        for script in &self.server_config.post_deploy_commands {
            debug!(
                "Executing post-deploy script for pack '{}': {:?}",
                pack_name, script
            );

            let exit_status = run_post_deploy_script(script.clone()).await?;

            ensure!(
                exit_status.success(),
                "Post-deploy script {:?} failed with {}",
                script,
                exit_status
            );

            debug!(
                "Post-deploy script for pack '{}' finished successfully: {:?}",
                pack_name, script
            );
        }

        Ok(())
    }

    // Creates symlinks to the keys in the server folder
    async fn symlink_keys(
        &self,
        keys: impl Iterator<Item = PathBuf>,
        server_path: &Path,
    ) -> anyhow::Result<()> {
        let key_dir = server_path.join("keys");

        if !fs::try_exists(&key_dir).await? {
            return Err(anyhow!(
                "Keys directory does not exist at {:?}. Cannot create symlinks.",
                key_dir
            ));
        }

        for key in keys {
            debug!(
                "Creating symlink for key at {:?} in server keys directory at {:?}",
                key, key_dir
            );

            let key_name = key.file_name().ok_or(anyhow!(
                "Failed to get file name for key at {:?}. Cannot create symlink.",
                key
            ))?;
            let dest_path = key_dir.join(key_name);

            create_or_recreate_symlink(&key, &dest_path)?;

            #[cfg(unix)]
            {
                std::os::unix::fs::lchown(
                    &dest_path,
                    self.server_config.file_owner_uid,
                    self.server_config.file_group_gid,
                )
                .context(anyhow!("Failed to change ownership for {:?}", key))?;
            }
        }

        Ok(())
    }

    async fn symlink_missions(&self, pack_name: &str, server_path: &Path) -> anyhow::Result<()> {
        // First remove all exising symlinks that link to our repo
        let server_missions_folder = server_path.join("mpmissions");

        let mut entries = fs::read_dir(&server_missions_folder)
            .await
            .context(anyhow!(
                "Failed to read server missions folder at {:?}",
                server_missions_folder
            ))?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.metadata().await?.is_symlink() {
                let target = fs::read_link(entry.path())
                    .await
                    .context(anyhow!("Failed to read symlink at {:?}", entry.path()))?;

                if target.starts_with(self.get_repo_path()) {
                    debug!(
                        "Removing existing symlink at {:?} that points to repo at {:?}",
                        entry.path(),
                        target
                    );
                    fs::remove_file(entry.path())
                        .await
                        .context(anyhow!("Failed to remove symlink at {:?}", entry.path()))?;
                }
            }
        }

        // Now create new symlinks for all missions in the repo
        let repo_missions_folder = self
            .get_repo_path()
            .join(pack_name)
            .join(MP_MISSIONS_DIR_NAME);

        let mut entries = fs::read_dir(&repo_missions_folder).await.context(anyhow!(
            "Failed to read repo missions folder at {:?}",
            repo_missions_folder
        ))?;

        while let Some(entry) = entries.next_entry().await? {
            let dest_path = server_missions_folder.join(entry.file_name());

            create_or_recreate_symlink(&entry.path(), &dest_path)?;

            #[cfg(unix)]
            {
                std::os::unix::fs::lchown(
                    &dest_path,
                    self.server_config.file_owner_uid,
                    self.server_config.file_group_gid,
                )
                .context(anyhow!("Failed to change ownership for {:?}", key))?;
            }
        }

        Ok(())
    }
}

/// Runs a post-deploy script to completion. `run_script` only offers a blocking
/// wait, so the whole run happens on the blocking thread pool.
async fn run_post_deploy_script(script: String) -> anyhow::Result<std::process::ExitStatus> {
    tokio::task::spawn_blocking(move || {
        let options = ScriptOptions::new();

        let args = vec![];

        let mut child = run_script::spawn(&script, &args, &options)
            .context(anyhow!("Failed to spawn post-deploy script: {:?}", script))?;

        child.wait().context(anyhow!(
            "Failed to wait for post-deploy script to finish: {:?}",
            script
        ))
    })
    .await?
}

// Looks in the keys folder for any key and returns their paths.
//
// The returned paths are always absolute: each found `.bikey` file is
// canonicalized. Note that `addon_path` itself is repo-root-relative (as
// produced by `resolve_addons`), so the lookup of `<addon_path>/keys` resolves
// against the process working directory.
async fn get_path_to_keys(addon_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let keys_dir = addon_path.join("keys");

    if !fs::try_exists(&keys_dir).await? {
        warn!(
            "Keys directory does not exist at {:?}. Returning empty list.",
            keys_dir
        );
        return Ok(vec![]);
    }

    let mut files = fs::read_dir(&keys_dir)
        .await
        .context(anyhow!("Failed to read keys directory at {:?}", keys_dir))?;

    let mut keys = Vec::new();
    while let Ok(Some(entry)) = files.next_entry().await {
        let path = entry.path();
        if is_file(&path).await && path.extension().is_some_and(|ext| ext == "bikey") {
            keys.push(PathBuf::from(canonicalize_and_clean_path(path)?));
        }
    }

    if keys.is_empty() {
        warn!(
            "No .bikey files found in keys directory at {:?}. Returning empty list.",
            keys_dir
        );
    }

    debug!(
        "Found {} .bikey files in keys directory at {:?}: {:?}",
        keys.len(),
        keys_dir,
        keys
    );

    Ok(keys)
}

#[cfg(test)]
mod tests {
    use crate::handle::reading::get_repo_info::GetRepoInfo;
    use crate::handle::server_repo_handle::ServerRepoHandle;
    use crate::models::pack::addon::AddonSettings;
    use crate::models::pack::pack_config::PackConfig;
    use crate::models::repo::repo_config::RepoConfig;
    use crate::util::test_utils::TestTempDir;
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;

    struct Fixture {
        _tmp: TestTempDir,
        server: ServerRepoHandle,
        server_dir: PathBuf,
        repo_path: PathBuf,
    }

    impl Fixture {
        async fn new(key: &str) -> Self {
            let tmp = TestTempDir::new(key);
            let server_dir = tmp.path().join("server");
            fs::create_dir_all(&server_dir).unwrap();
            fs::create_dir_all(server_dir.join("keys")).unwrap();
            fs::create_dir_all(server_dir.join("mpmissions")).unwrap();

            let mut packs = HashSet::new();
            packs.insert("core".to_string());

            let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), packs);
            let mut server = ServerRepoHandle::create(tmp.path(), repo_config)
                .await
                .unwrap();
            let repo_path = server.get_repo_path().to_path_buf();

            let mut pack =
                PackConfig::new("core".to_string(), "core pack".to_string(), vec![], None);
            pack.addons
                .insert("@addon1".to_string(), AddonSettings { is_optional: false });
            pack.init_source_on_fs(&repo_path).await.unwrap();

            fs::create_dir_all(repo_path.join("core/addons/@addon1")).unwrap();

            server.server_config.server_dir = Some(server_dir.clone());

            Self {
                _tmp: tmp,
                server,
                server_dir,
                repo_path,
            }
        }

        // Absolute path to a file inside the fixture's server dir (which lives
        // under the system temp directory).
        fn script_path(&self, name: &str) -> PathBuf {
            self.server_dir.join(name)
        }

        // Absolute path to the `pamm/repo` symlink inside the fixture's server dir.
        fn pamm_symlink(&self) -> PathBuf {
            self.server_dir.join("pamm").join("repo")
        }
    }

    #[tokio::test]
    async fn deploy_pack_writes_template_and_runs_post_deploy_commands() {
        let mut fx = Fixture::new("pamm_deploy_pack_writes_template_and_runs_commands").await;

        let template_path = fx.script_path("deploy-template.txt");
        fs::write(
            &template_path,
            "start {DEPLOYED_PACK}::{MOD_LAUNCH_PARAM} end",
        )
        .unwrap();

        let command_log = fx.script_path("post-deploy.log");
        let command_1 = format!("echo first > \"{}\"", command_log.display());
        let command_2 = format!("echo second >> \"{}\"", command_log.display());

        fx.server.server_config.script_templates.insert(
            template_path.clone(),
            fs::read_to_string(&template_path).unwrap(),
        );
        fx.server
            .server_config
            .post_deploy_commands
            .extend([command_1, command_2]);

        fx.server.deploy_pack("core").await.unwrap();

        let expected_mod_param = format!(
            "\"-mod={}\"",
            PathBuf::new()
                .join("pamm")
                .join("repo")
                .join("core")
                .join("addons")
                .join("@addon1")
                .to_string_lossy()
        );

        assert!(fx.server_dir.join("pamm").is_dir());
        assert!(fx.pamm_symlink().exists());
        assert!(
            fs::symlink_metadata(fx.pamm_symlink())
                .unwrap()
                .file_type()
                .is_symlink()
        );

        assert_eq!(
            fs::read_to_string(&template_path).unwrap(),
            format!("start core::{} end", expected_mod_param)
        );

        let log_contents = fs::read_to_string(&command_log).unwrap();
        assert_eq!(
            log_contents.lines().map(str::trim_end).collect::<Vec<_>>(),
            vec!["first", "second"]
        );

        assert!(fx.repo_path.join("core/addons/@addon1").is_dir());
    }

    #[tokio::test]
    async fn deploy_pack_skips_templates_without_placeholder() {
        let mut fx = Fixture::new("pamm_deploy_pack_skips_templates_without_placeholder").await;

        let template_path = fx.script_path("deploy-template.txt");
        fs::write(&template_path, "unchanged template").unwrap();
        fx.server.server_config.script_templates.insert(
            template_path.clone(),
            fs::read_to_string(&template_path).unwrap(),
        );

        fx.server.deploy_pack("core").await.unwrap();

        assert_eq!(
            fs::read_to_string(&template_path).unwrap(),
            "unchanged template"
        );
    }

    #[tokio::test]
    async fn deploy_pack_requires_server_directory() {
        let tmp = TestTempDir::new("pamm_deploy_pack_requires_server_directory");
        let mut packs = HashSet::new();
        packs.insert("core".to_string());

        let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), packs);
        let server = ServerRepoHandle::create(tmp.path(), repo_config)
            .await
            .unwrap();

        let mut pack = PackConfig::new("core".to_string(), "core pack".to_string(), vec![], None);
        pack.addons
            .insert("@addon1".to_string(), AddonSettings { is_optional: false });
        pack.init_source_on_fs(server.get_repo_path())
            .await
            .unwrap();
        fs::create_dir_all(server.get_repo_path().join("core/addons/@addon1")).unwrap();

        let err = server.deploy_pack("core").await.unwrap_err().to_string();
        assert!(
            err.contains("Server directory is not set"),
            "unexpected error: {}",
            err
        );
    }
}
