use pamm_lib::handle::repo_handle::RepoHandle;
use std::path::Path;

pub fn quick_check(pack_name: String, repo_path: String) -> anyhow::Result<bool> {
    let repo_dir = Path::new(&repo_path);

    let handle = RepoHandle::open(repo_dir)?;

    handle.quick_check_pack_up_to_date(&pack_name)
}
