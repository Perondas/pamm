use crate::domain::pack::pack_user_settings::PackUserSettings;

pub trait SavePackSettings {
    /// The returned future does not borrow the arguments (`use<Self>`), so
    /// callers can collect several of them and await them together.
    fn save_pack_settings(
        &self,
        pack_name: &str,
        settings: &PackUserSettings,
    ) -> impl Future<Output = anyhow::Result<()>> + Send + use<Self>;
}
