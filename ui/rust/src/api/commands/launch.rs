use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::path::Path;

pub fn launch(repo_dir: String, pack_name: String, launch_type: LaunchType) -> anyhow::Result<()> {
    let repo_dir = Path::new(&repo_dir);

    let handle = ClientRepoHandle::open(repo_dir)?;

    #[allow(unreachable_patterns)]
    match launch_type {
        LaunchType::Steam => handle.launch_via_steam(&pack_name),
        #[cfg(target_os = "windows")]
        LaunchType::File => handle.launch_via_executable(&pack_name),
        _ => handle.launch_via_steam(&pack_name),
    }
}

pub enum LaunchType {
    Steam,
    File,
}
