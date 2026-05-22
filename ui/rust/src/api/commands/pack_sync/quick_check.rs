use pamm_lib::handle::client::client_repo_handle::ClientRepoHandle;
use std::path::Path;

pub fn quick_check(pack_name: String, repo_path: String) -> anyhow::Result<bool> {
    let repo_dir = Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_dir)?;

    handle.quick_check_pack_up_to_date(&pack_name)
}
