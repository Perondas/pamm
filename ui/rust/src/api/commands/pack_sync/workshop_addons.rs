use pamm_lib::application::addons::workshop::LocateWorkshopAddons;
use pamm_lib::application::repo::client::ClientRepoHandle;
use std::path::Path;

/// A workshop addon the pack needs, but that could not be found on this machine.
pub struct WorkshopAddonNotFound {
    pub name: String,
    pub pack_name: String,
    /// The Steam workshop page, where the user can subscribe to the addon.
    pub url: String,
    /// The same page, opened in the Steam client.
    pub steam_url: String,
}

/// Gets the workshop addons launching the pack needs that cannot be found, so
/// the user can be pointed at them to subscribe.
#[err_wrap]
pub async fn workshop_addons_not_found(
    repo_path: String,
    pack_name: String,
) -> anyhow::Result<Vec<WorkshopAddonNotFound>> {
    let handle = ClientRepoHandle::open(Path::new(&repo_path)).await?;

    let not_found = handle.workshop_addons_not_found(&pack_name).await?;

    Ok(not_found
        .into_iter()
        .filter_map(|addon| {
            let url = addon.workshop_url()?;
            let steam_url = addon.workshop_steam_url()?;
            Some(WorkshopAddonNotFound {
                name: addon.name,
                pack_name: addon.pack_name,
                url,
                steam_url,
            })
        })
        .collect())
}
