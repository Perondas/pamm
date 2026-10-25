use crate::handle::externals::external_addon::ExternalAddon;
use crate::handle::reading::get_pack::GetPack;
use crate::util::dirs::find_workshop_folder::find_workshop_folder;
use std::collections::HashSet;
use std::fs::read_dir;

pub trait LoadExternals {
    fn load_externals(
        &self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<Vec<ExternalAddon>>> + Send;
}

impl<T> LoadExternals for T
where
    T: GetPack + Sync,
{
    async fn load_externals(&self, pack_name: &str) -> anyhow::Result<Vec<ExternalAddon>> {
        let (_, settings) = self.get_pack_with_settings(pack_name).await?;

        let mut externals = settings.external_addons.clone();

        let ws_addons = find_workshop_addons().unwrap_or_default();

        externals.extend(ws_addons);

        externals.retain(|addon| addon.path.exists());

        Ok(externals.into_iter().collect())
    }
}

fn find_workshop_addons() -> anyhow::Result<HashSet<ExternalAddon>> {
    let mut externals = HashSet::new();
    if let Ok(path) = find_workshop_folder() {
        for entry in read_dir(path)? {
            let addon = ExternalAddon::new(entry?.path())?.found_in_workshop();
            externals.insert(addon);
        }
    }
    Ok(externals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::mock_handle::MockHandle;
    use crate::models::pack::pack_config::PackConfig;
    use crate::models::pack::pack_user_settings::PackUserSettings;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_load_externals() {
        let mut mock = MockHandle::new();

        let config = PackConfig::new(
            "test_pack".to_string(),
            "desc".to_string(),
            vec![],
            None,
            vec![],
        );

        let mut settings = PackUserSettings::default();
        let external = ExternalAddon {
            path: PathBuf::from("/some/path"),
            name: Some("addon".to_string()),
            enabled: true,
            found_in_workshop: false,
        };
        settings.external_addons.insert(external.clone());

        mock.expect_get_pack_with_settings()
            .with(mockall::predicate::eq("test_pack"))
            .returning(move |_| Ok((config.clone(), settings.clone())));

        let (_, settings) = mock.get_pack_with_settings("test_pack").await.unwrap();
        let externals = settings.external_addons.clone();
        assert_eq!(externals.len(), 1);
        let ext = externals.iter().next().unwrap();
        assert_eq!(ext.path, PathBuf::from("/some/path"));
        assert_eq!(ext.name, Some("addon".to_string()));
        assert!(ext.enabled);
        assert!(!ext.found_in_workshop);
    }
}
