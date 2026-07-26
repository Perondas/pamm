use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::reading::get_pack::GetPack;

/// The subset of a pack's config the UI needs to display it.
pub struct PackDisplayInfo {
    pub name: String,
    pub description: String,
    /// File name of the pack's icon inside the repo's `media/` directory.
    pub icon: Option<String>,
    /// File name of the pack's banner inside the repo's `media/` directory.
    pub banner: Option<String>,
}

pub fn load_pack_display(
    repo_path: String,
    pack_name: String,
) -> anyhow::Result<PackDisplayInfo> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path)?;
    let pack = handle.get_pack(&pack_name)?;
    let customization = pack.customization.unwrap_or_default();

    Ok(PackDisplayInfo {
        name: pack.name,
        description: pack.description,
        icon: customization.icon,
        banner: customization.banner,
    })
}
