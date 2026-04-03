use crate::handle::repo_handle::RepoHandle;
use crate::io::name_consts::get_pack_addon_directory_name;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::OptionalAddonSetting;
use std::collections::HashMap;
use std::path::PathBuf;

impl RepoHandle {
    pub(in crate::handle) fn resolve_optionals(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<Vec<PathBuf>> {
        let (pack_config, settings) = self.get_pack_with_settings(pack_name)?;

        self.resolve_optionals_recursive(pack_config, settings.enabled_optionals)
    }

    fn resolve_optionals_recursive(
        &self,
        config: PackConfig,
        optionals: HashMap<String, OptionalAddonSetting>,
    ) -> anyhow::Result<Vec<PathBuf>> {
        let mut res = Vec::new();

        let addon_dir = self
            .repo_path
            .join(get_pack_addon_directory_name(&config.name));

        for (optional, _) in &optionals {
            if config
                .addons
                .iter()
                .any(|(name, addon)| name == optional && addon.is_optional)
            {
                let optional_path = addon_dir.join(optional);
                res.push(optional_path);
            }
        }

        let mut others = if let Some(parent) = config.parent {
            let (pack_config, _) = self.get_pack_with_settings(&parent)?;
            self.resolve_optionals_recursive(pack_config, optionals)?
        } else {
            vec![]
        };

        res.append(&mut others);

        Ok(res)
    }
}
