use pamm_lib::handle::client::client_repo_handle::ClientRepoHandle;
use std::path::Path;

pub fn launch(repo_dir: String, pack_name: String) -> anyhow::Result<()> {
    let repo_dir = Path::new(&repo_dir);

    let handle = ClientRepoHandle::open(repo_dir)?;

    handle.launch_via_steam(&pack_name)
}
