use crate::application::addons::active::GetActiveAddons;
use crate::application::pack::read::get_pack::GetPack;
use crate::application::pack::read::get_repo_info::GetRepoInfo;
use crate::domain::addon::{AddonSource, PackAddon};

pub trait LocateWorkshopAddons {
    /// Gets the workshop addons launching the pack needs (including inherited
    /// ones, enabled optionals and the CDLC compat) that cannot be found on this
    /// machine, so the user can be pointed at them to subscribe.
    fn workshop_addons_not_found(
        &self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<Vec<PackAddon>>> + Send;
}

impl<T> LocateWorkshopAddons for T
where
    T: GetPack + GetRepoInfo + Sync,
{
    async fn workshop_addons_not_found(&self, pack_name: &str) -> anyhow::Result<Vec<PackAddon>> {
        let repo_path = self.get_repo_path();

        let mut not_found = Vec::new();
        for addon in self.get_launch_addons(pack_name, true).await? {
            if matches!(addon.source(), AddonSource::Workshop { .. })
                && addon.locate(repo_path)?.is_none()
            {
                not_found.push(addon);
            }
        }

        Ok(not_found)
    }
}
