use crate::models::pack::pack_user_settings::PackUserSettings;

#[cfg_attr(test, mockall::automock)]
pub trait SavePackSettings {
    fn save_pack_settings(
        &self,
        pack_name: &str,
        settings: &PackUserSettings,
    ) -> anyhow::Result<()>;
}
