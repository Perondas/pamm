use pamm_lib::handle::repo_handle::RepoHandle;

pub fn get_launch_params(repot_path: String, pack_name: String) -> anyhow::Result<Vec<String>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = RepoHandle::open(repot_path)?;

    handle.get_pack_launch_params(&pack_name)
}

pub fn set_launch_params(
    repot_path: String,
    pack_name: String,
    launch_params: Vec<String>,
) -> anyhow::Result<()> {
    let repot_path = std::path::Path::new(&repot_path);
    let handle = RepoHandle::open(repot_path)?;

    handle.set_pack_launch_params(&pack_name, launch_params)
}
