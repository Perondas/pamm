use crate::handle::repo_handle::RepoHandle;
use crate::io::name_consts::get_pack_addon_directory_name;
use log::{debug, trace};
use std::path::PathBuf;

impl RepoHandle {
    pub(in crate::handle) fn resolve_optionals_paths(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<Vec<PathBuf>> {
        let (config, settings) = self.get_pack_with_settings(pack_name)?;

        let mut res = Vec::new();

        let addon_dir = self
            .repo_path
            .join(get_pack_addon_directory_name(&config.name));

        for optional in &settings.enabled_optionals {
            if config
                .addons
                .get(optional)
                .is_some_and(|addon| addon.is_optional)
            {
                trace!(
                    "Optional addon '{}' is enabled for pack '{}'",
                    optional, config.name
                );
                let optional_path = addon_dir.join(optional);
                res.push(optional_path);
            }
        }

        debug!(
            "Optional addon paths for pack '{}': {:#?}",
            config.name, res
        );

        let mut others = if let Some(parent) = config.parent {
            self.resolve_optionals_paths(&parent)?
        } else {
            vec![]
        };

        debug!(
            "Parent optional addon paths for pack '{}': {:#?}",
            config.name, others
        );

        res.append(&mut others);

        Ok(res)
    }
}
