use crate::api::commands::optionals::optional_addon::OptionalAddon;
use anyhow::{anyhow, Context};
use pamm_lib::io::fs::fs_readable::NamedFSReadable;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_user_settings::PackUserSettings;

pub fn load_optionals(repot_path: String, pack_name: String) -> anyhow::Result<Vec<OptionalAddon>> {
    let repot_path = std::path::Path::new(&repot_path);

    let pack_config = PackConfig::read_from_named(repot_path, &pack_name)
        .context(anyhow!("Pack configuration not found"))?;

    let settings = PackUserSettings::read_from_named(repot_path, &pack_name)
        .context(anyhow!("Pack user settings not found"))?;

    Ok(pack_config
        .addons
        .iter()
        .filter(|(_, addon)| addon.is_optional)
        .map(|(name, _)| {
            OptionalAddon::new(name.clone(), settings.enabled_optionals.contains(name))
        })
        .collect())
}
