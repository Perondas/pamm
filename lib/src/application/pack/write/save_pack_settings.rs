use crate::domain::pack::pack_user_settings::PackUserSettings;

pub trait SavePackSettings {
    fn save_pack_settings<'a, 'b>(
        &'a self,
        pack_name: &'a str,
        settings: &'a PackUserSettings,
    ) -> impl Future<Output = anyhow::Result<()>> + Send + 'b + use<'b, Self>;
}
