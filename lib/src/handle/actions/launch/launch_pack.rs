use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_canonical_addon_paths::GetAddonPaths;
use log::{debug, info};

#[derive(Debug, Clone)]
pub enum LaunchMode {
    Steam,
    #[cfg(target_os = "windows")]
    Executable,
}
#[derive(Debug)]
pub struct LaunchParams {
    mode: LaunchMode,
    disable_optionals: bool,
}

impl LaunchParams {
    pub fn new(mode: LaunchMode, disable_optionals: bool) -> Self {
        Self {
            mode,
            disable_optionals,
        }
    }
}

impl ClientRepoHandle {
    pub fn launch_pack(
        &self,
        pack_name: &str,
        params: &LaunchParams,
    ) -> anyhow::Result<()> {
        info!("Launching pack with params: '{:#?}'", params);

        // On linux we need to have the load path be in the Arma directory.
        let addon_paths = cfg_select! {
            target_os = "linux" => self.get_linux_addon_paths(pack_name, params.disable_optionals),
            _ => self.get_canonical_addon_paths(pack_name, params.disable_optionals)
        }?;

        debug!(
            "Resolved {} addon path(s) for pack '{}'",
            addon_paths.len(),
            pack_name
        );
        
        match params.mode {
            LaunchMode::Steam => self.launch_via_steam(pack_name, &addon_paths),
            #[cfg(target_os = "windows")]
            LaunchMode::Executable => self.launch_via_executable(pack_name, &addon_paths),
        }
    }
}
