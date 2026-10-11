use crate::handle::optionals::optional_addon::OptionalAddon;
use crate::handle::reading::get_pack::GetPack;

impl<T> LoadOptionals for T
where
    T: GetPack + Sync,
{
    async fn load_optionals(&self, pack_name: &str) -> anyhow::Result<Vec<OptionalAddon>> {
        let mut optionals = Vec::new();
        let mut current = Some(pack_name.to_string());

        // Own optionals first, then each ancestor's in turn.
        while let Some(pack_name) = current {
            let (pack_config, settings) = self.get_pack_with_settings(&pack_name).await?;

            optionals.extend(
                pack_config
                    .addons
                    .iter()
                    .filter(|(_, addon)| addon.is_optional)
                    .map(|(name, _)| {
                        OptionalAddon::new(name.clone(), settings.enabled_optionals.contains(name))
                    }),
            );

            current = pack_config.parent;
        }

        Ok(optionals)
    }
}

pub trait LoadOptionals {
    fn load_optionals(
        &self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<Vec<OptionalAddon>>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::mock_handle::{MockHandle, MockHandleExt};

    #[tokio::test]
    async fn test_load_optionals() {
        let mut mock_get_pack = MockHandle::new();

        mock_get_pack.mock_pack(
            "test_pack",
            None,
            &["@opt_addon"],
            &["@req_addon"],
            &["@opt_addon"],
        );

        let optionals = mock_get_pack.load_optionals("test_pack").await.unwrap();

        assert_eq!(optionals.len(), 1);
        assert_eq!(optionals[0].name, "@opt_addon");
        assert!(optionals[0].enabled);
    }

    #[tokio::test]
    async fn test_load_optionals_with_parent() {
        let mut mock_get_pack = MockHandle::new();

        mock_get_pack.mock_pack(
            "child_pack",
            Some("parent_pack"),
            &["@child_opt"],
            &[],
            &["@child_opt"],
        );

        mock_get_pack.mock_pack("parent_pack", None, &["@parent_opt"], &[], &[]);

        let mut optionals = mock_get_pack.load_optionals("child_pack").await.unwrap();

        // Sort to ensure consistent order for assertions
        optionals.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(optionals.len(), 2);
        assert_eq!(optionals[0].name, "@child_opt");
        assert!(optionals[0].enabled);
        assert_eq!(optionals[1].name, "@parent_opt");
        assert!(!optionals[1].enabled);
    }
}
