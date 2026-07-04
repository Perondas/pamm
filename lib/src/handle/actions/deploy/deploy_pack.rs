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
