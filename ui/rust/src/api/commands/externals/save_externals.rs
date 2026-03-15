use pamm_lib::handle::externals::external_addon::ExternalAddon;
use pamm_lib::handle::repo_handle::RepoHandle;

pub fn save_externals(
    repo_path: String,
    pack_name: String,
    externals: Vec<ExternalAddon>,
) -> anyhow::Result<()> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = RepoHandle::open(repo_path)?;

    handle.save_externals(&pack_name, &externals)
}
