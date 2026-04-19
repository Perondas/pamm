use crate::handle::repo_handle::RepoHandle;
use crate::models::pack::pack_user_settings::PackUserSettings;

#[cfg_attr(test, mockall::automock)]
pub trait SavePackSettings {
    fn save_pack_settings(
        &self,
        pack_name: &str,
        settings: &PackUserSettings,
    ) -> anyhow::Result<()>;
}

impl SavePackSettings for RepoHandle {
    fn save_pack_settings(
        &self,
        pack_name: &str,
        settings: &PackUserSettings,
    ) -> anyhow::Result<()> {
        self.write_named(settings, pack_name)
    }
}
