use crate::api::commands::optionals::optional_addon::OptionalAddon;
use anyhow::{anyhow, Context};
use pamm_lib::io::fs::fs_readable::NamedFSReadable;
use pamm_lib::io::fs::fs_writable::NamedFSWritable;
use pamm_lib::models::pack::pack_config::PackConfig;
use pamm_lib::models::pack::pack_user_settings::PackUserSettings;

pub fn save_optionals(
    repot_path: String,
    pack_name: String,
    optionals: Vec<OptionalAddon>,
) -> anyhow::Result<()> {
    let repot_path = std::path::Path::new(&repot_path);

    let pack_config = PackConfig::read_from_named(repot_path, &pack_name)
        .context(anyhow!("Pack configuration not found"))?;

    let enabled = optionals
        .into_iter()
        .filter(|optional| optional.enabled)
        .filter(|optional| {
            pack_config
                .addons
                .get(&optional.name)
                .is_some_and(|addon| addon.is_optional)
        })
        .map(|optional| optional.name)
        .collect();

    let mut settings = PackUserSettings::read_from_named(repot_path, &pack_name)
        .context(anyhow!("Pack user settings not found"))?;

    settings.enabled_optionals = enabled;

    settings.write_to_named(repot_path, &pack_name)?;

    Ok(())
}
