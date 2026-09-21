use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_canonical_addon_paths::GetAddonPaths;
use log::{debug, info};

/// What a launch left behind, for callers that want to tell the user about it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LaunchOutcome {
    /// The preset file the launch wrote, where the platform uses one.
    pub preset_path: Option<String>,
}

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
    ) -> anyhow::Result<LaunchOutcome> {
        info!("Launching pack with params: '{:#?}'", params);

        // Absolute host paths on every platform. Canonicalizing resolves any
        // symlinks along the way, which matters on Linux: a Flatpak filesystem
        // grant has to cover a symlink's target, not the link.
        let addon_paths = self.get_canonical_addon_paths(pack_name, params.disable_optionals)?;

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
