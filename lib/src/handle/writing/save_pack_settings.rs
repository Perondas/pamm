use crate::models::pack::pack_user_settings::PackUserSettings;

pub trait SavePackSettings {
    fn save_pack_settings(
        &self,
        pack_name: &str,
        settings: &PackUserSettings,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}
