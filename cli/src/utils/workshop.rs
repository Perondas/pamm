use pamm_lib::application::addons::workshop::LocateWorkshopAddons;
use pamm_lib::application::repo::client::ClientRepoHandle;

/// Points the user at the workshop addons the pack needs but that cannot be
/// found, as those are not synced by pamm but subscribed to on Steam.
pub async fn report_workshop_addons_not_found(
    handle: &ClientRepoHandle,
    pack_name: &str,
) -> anyhow::Result<()> {
    let not_found = handle.workshop_addons_not_found(pack_name).await?;

    if not_found.is_empty() {
        return Ok(());
    }

    println!(
        "\nThe following workshop addons were not found. Subscribe to them on the Steam workshop:"
    );

    for addon in &not_found {
        if let Some(url) = addon.workshop_url() {
            println!("  {}: {}", addon.name, url);
        }
    }

    Ok(())
}
