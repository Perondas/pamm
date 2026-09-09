use pamm_lib::handle::actions::launch::launch::{LaunchMode, LaunchParams};
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::path::Path;

pub fn launch(
    repo_dir: String,
    pack_name: String,
    launch_type: LaunchType,
    disable_optionals: bool,
) -> anyhow::Result<()> {
    let repo_dir = Path::new(&repo_dir);

    let handle = ClientRepoHandle::open(repo_dir)?;

    let launch_params = LaunchParams::new(launch_type.into(), disable_optionals);

    handle.launch_pack(&pack_name, &launch_params)
}

pub enum LaunchType {
    Steam,
    File,
}

impl From<LaunchType> for LaunchMode {
    fn from(value: LaunchType) -> Self {
        match value {
            LaunchType::Steam => LaunchMode::Steam,
            #[cfg(target_os = "windows")]
            LaunchType::File => LaunchMode::Executable,
        }
    }
}
