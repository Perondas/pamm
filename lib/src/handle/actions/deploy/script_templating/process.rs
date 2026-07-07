use super::context::ScriptTemplateContext;
use super::deployed_pack_name::DeployedPackNameReplacementStrategy;
use super::mod_launch_param::ModLaunchParamReplacementStrategy;
use super::strategy::ScriptTemplateReplacementStrategy;
use crate::handle::actions::deploy::script_templating::datestamp::DatestampReplacementStrategy;
use crate::handle::server_repo_handle::ServerRepoHandle;
use anyhow::{Context, anyhow};
use log::debug;
use std::fs;
use std::path::Path;

impl ServerRepoHandle {
    pub(crate) fn process_script_templates(
        &self,
        pack_name: &str,
        mod_launch_param: &str,
    ) -> anyhow::Result<()> {
        let strategies: [&dyn ScriptTemplateReplacementStrategy; 3] = [
            &ModLaunchParamReplacementStrategy,
            &DeployedPackNameReplacementStrategy,
            &DatestampReplacementStrategy,
        ];

        for (path, template) in &self.server_config.script_templates {
            let script_name = path.to_string_lossy();

            let context = ScriptTemplateContext::new(pack_name, &script_name, mod_launch_param);
            let mut script_content = template.to_owned();

            render_script_template(&context, &mut script_content, &strategies);

            fs::write(path, &script_content)
                .context(anyhow!("Failed to write script file at {:?}", path))?;

            self.make_executable(path).context(anyhow!(
                "Failed to make script file executable at {:?}",
                path
            ))?;

            debug!(
                "Deployed script for pack '{}' at {:?} with mod launch parameter: {}",
                pack_name, path, mod_launch_param
            );
        }
        Ok(())
    }

    #[cfg(unix)]
    fn make_executable(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        use std::os::unix::fs::PermissionsExt;

        debug!("Making script {:#?} executable", path);

        let mut perms = fs::metadata(&path)
            .context(anyhow!(
                "Failed to get metadata for script file at {:?}",
                path
            ))?
            .permissions();

        perms.set_mode(perms.mode() | 0o111); // Add execute permissions for user, group, and others

        fs::set_permissions(&path, perms).context(anyhow!("Failed to set mode for {:?}", path))?;

        std::os::unix::fs::chown(
            &path,
            self.server_config.file_owner_uid,
            self.server_config.file_group_gid,
        )
        .context(anyhow!("Failed to change ownership for {:?}", path))
    }

    #[cfg(windows)]
    fn make_executable(&self, _path: impl AsRef<Path>) -> anyhow::Result<()> {
        // Do nothing
        Ok(())
    }
}

fn render_script_template(
    context: &ScriptTemplateContext<'_>,
    script_content: &mut String,
    strategies: &[&dyn ScriptTemplateReplacementStrategy],
) {
    for strategy in strategies {
        strategy.apply(context, script_content);
    }
}
