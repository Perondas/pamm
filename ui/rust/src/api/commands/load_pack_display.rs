use pamm_lib::application::pack::read::get_pack::GetPack;
use pamm_lib::application::repo::client::ClientRepoHandle;

/// The subset of a pack's config the UI needs to display it.
pub struct PackDisplayInfo {
    pub name: String,
    pub description: String,
    /// File name of the pack's icon inside the repo's `media/` directory.
    pub icon: Option<String>,
}

#[err_wrap]
pub async fn load_pack_display(
    repo_path: String,
    pack_name: String,
) -> anyhow::Result<PackDisplayInfo> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path).await?;
    let pack = handle.get_pack(&pack_name).await?;
    let customization = pack.customization.unwrap_or_default();

    Ok(PackDisplayInfo {
        name: pack.name,
        description: pack.description,
        icon: customization.icon,
    })
}
