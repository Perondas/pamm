use crate::handle::optionals::optional_addon::OptionalAddon;
use crate::handle::reading::get_pack::GetPack;

impl<T> LoadOptionals for T
where
    T: GetPack,
{
    fn load_optionals(&self, pack_name: &str) -> anyhow::Result<Vec<OptionalAddon>> {
        let (pack_config, settings) = self.get_pack_with_settings(pack_name)?;

        let parent_optionals = if let Some(parent) = &pack_config.parent {
            self.load_optionals(parent)?
        } else {
            vec![]
        };

        Ok(pack_config
            .addons
            .iter()
            .filter(|(_, addon)| addon.is_optional)
            .map(|(name, _)| {
                OptionalAddon::new(name.clone(), settings.enabled_optionals.contains(name))
            })
            .chain(parent_optionals)
            .collect())
    }
}

pub trait LoadOptionals {
    fn load_optionals(&self, pack_name: &str) -> anyhow::Result<Vec<OptionalAddon>>;
}
