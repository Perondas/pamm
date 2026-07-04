use crate::handle::addons::ResolveAddons;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::server_repo_handle::ServerRepoHandle;
use crate::io::fs::util::symlink::create_or_recreate_symlink;
use anyhow::{Context, anyhow, ensure};
use log::{debug, warn};
use run_script::ScriptOptions;
use std::fs;

const MOD_LAUNCH_PARAM_PLACEHOLDER: &str = "{MOD_LAUNCH_PARAM}";

impl ServerRepoHandle {
    pub fn deploy_pack(&self, pack_name: &str) -> anyhow::Result<()> {
        let pamm_dir = self
            .server_config
            .server_dir
            .as_ref()
            .ok_or(anyhow!(
                "Server directory is not set in the server configuration.",
            ))?
            .join("pamm");

        fs::create_dir(&pamm_dir)
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

        let addon_paths = self
            .resolve_addons(pack_name)?
            .iter()
            .map(|p| {
                p.to_str()
                    .map(|s| s.to_string())
                    .context(anyhow!("Failed to convert path to string: {:?}", p))
            })
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .map(|rel| format!("pamm/{}/{}", self.get_config().name, rel))
            .collect::<Vec<_>>();

        let mod_launch_param = format!("\"-mod={}\"", addon_paths.join(";"));

        debug!(
            "Deploying pack '{}' with mod launch parameter: {}",
            pack_name, mod_launch_param
        );

        for (path, template) in &self.server_config.script_templates {
            ensure!(path.is_file(), "Script path is not a file: {:?}", path);

            if !template.contains(MOD_LAUNCH_PARAM_PLACEHOLDER) {
                warn!(
                    "Template for script {:?} does not contain the placeholder {}. Skipping.",
                    path, MOD_LAUNCH_PARAM_PLACEHOLDER
                );
                continue;
            }

            let script_content = template.replace(MOD_LAUNCH_PARAM_PLACEHOLDER, &mod_launch_param);

            fs::write(path, script_content)
                .context(anyhow!("Failed to write script file at {:?}", path))?;

            debug!(
                "Deployed script for pack '{}' at {:?} with mod launch parameter: {}",
                pack_name, path, mod_launch_param
            );
        }

        for script in &self.server_config.post_deploy_commands {
            debug!(
                "Executing post-deploy script for pack '{}': {:?}",
                pack_name, script
            );

            let options = ScriptOptions::new();

            let args = vec![];

            let mut child = run_script::spawn(script, &args, &options)
                .context(anyhow!("Failed to spawn post-deploy script: {:?}", script))?;

            child.wait().context(anyhow!(
                "Failed to wait for post-deploy script to finish: {:?}",
                script
            ))?;

            debug!(
                "Post-deploy script for pack '{}' finished successfully: {:?}",
                pack_name, script
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MOD_LAUNCH_PARAM_PLACEHOLDER;
    use crate::handle::reading::get_repo_info::GetRepoInfo;
    use crate::handle::server_repo_handle::ServerRepoHandle;
    use crate::io::fs::fs_writable::NamedFSWritable;
    use crate::io::name_consts::get_pack_addon_directory_name;
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
        fn new(key: &str) -> Self {
            let tmp = TestTempDir::new(key);
            let server_dir = tmp.path().join("server");
            fs::create_dir_all(&server_dir).unwrap();

            let mut packs = HashSet::new();
            packs.insert("core".to_string());

            let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), packs);
            let mut server = ServerRepoHandle::create(tmp.path(), repo_config).unwrap();
            let repo_path = server.get_repo_path().to_path_buf();

            let mut pack = PackConfig::new(
                "core".to_string(),
                "core pack".to_string(),
                vec![],
                vec![],
                None,
            );
            pack.addons
                .insert("@addon1".to_string(), AddonSettings { is_optional: false });
            pack.write_to_named(&repo_path, "core").unwrap();

            fs::create_dir_all(
                repo_path
                    .join(get_pack_addon_directory_name("core"))
                    .join("@addon1"),
            )
            .unwrap();

            server.server_config.server_dir = Some(server_dir.clone());

            Self {
                _tmp: tmp,
                server,
                server_dir,
                repo_path,
            }
        }

        fn script_path(&self, name: &str) -> PathBuf {
            self.server_dir.join(name)
        }

        fn pamm_symlink(&self) -> PathBuf {
            self.server_dir.join("pamm").join("repo")
        }
    }

    #[test]
    fn deploy_pack_writes_template_and_runs_post_deploy_commands() {
        let mut fx = Fixture::new("pamm_deploy_pack_writes_template_and_runs_commands");

        let template_path = fx.script_path("deploy-template.txt");
        fs::write(
            &template_path,
            format!("start {MOD_LAUNCH_PARAM_PLACEHOLDER} end"),
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

        fx.server.deploy_pack("core").unwrap();

        let expected_mod_param = format!(
            "\"-mod={}\"",
            format!(
                "pamm/repo/{}",
                PathBuf::from(get_pack_addon_directory_name("core"))
                    .join("@addon1")
                    .to_string_lossy()
            )
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
            format!("start {} end", expected_mod_param)
        );

        let log_contents = fs::read_to_string(&command_log).unwrap();
        assert_eq!(
            log_contents.lines().map(str::trim_end).collect::<Vec<_>>(),
            vec!["first", "second"]
        );

        assert!(
            fx.repo_path
                .join("core_pack_addons")
                .join("@addon1")
                .is_dir()
        );
    }

    #[test]
    fn deploy_pack_skips_templates_without_placeholder() {
        let mut fx = Fixture::new("pamm_deploy_pack_skips_templates_without_placeholder");

        let template_path = fx.script_path("deploy-template.txt");
        fs::write(&template_path, "unchanged template").unwrap();
        fx.server.server_config.script_templates.insert(
            template_path.clone(),
            fs::read_to_string(&template_path).unwrap(),
        );

        fx.server.deploy_pack("core").unwrap();

        assert_eq!(
            fs::read_to_string(&template_path).unwrap(),
            "unchanged template"
        );
    }

    #[test]
    fn deploy_pack_requires_server_directory() {
        let tmp = TestTempDir::new("pamm_deploy_pack_requires_server_directory");
        let mut packs = HashSet::new();
        packs.insert("core".to_string());

        let repo_config = RepoConfig::new("repo".to_string(), "desc".to_string(), packs);
        let server = ServerRepoHandle::create(tmp.path(), repo_config).unwrap();

        let mut pack = PackConfig::new(
            "core".to_string(),
            "core pack".to_string(),
            vec![],
            vec![],
            None,
        );
        pack.addons
            .insert("@addon1".to_string(), AddonSettings { is_optional: false });
        pack.write_to_named(server.get_repo_path(), "core").unwrap();
        fs::create_dir_all(
            server
                .get_repo_path()
                .join(get_pack_addon_directory_name("core"))
                .join("@addon1"),
        )
        .unwrap();

        let err = server.deploy_pack("core").unwrap_err().to_string();
        assert!(
            err.contains("Server directory is not set"),
            "unexpected error: {}",
            err
        );
    }
}
