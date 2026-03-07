use pamm_lib::handle::repo_handle::RepoHandle;
use std::path::Path;

pub fn launch(repo_dir: String, pack_name: String) -> anyhow::Result<()> {
    let repo_dir = Path::new(&repo_dir);

    let handle = RepoHandle::open(repo_dir)?;

    handle.launch_via_steam(&pack_name)
}
