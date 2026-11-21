pub const ARMA_APP_ID: u32 = 107_410;

pub mod arma_install;
#[cfg(target_os = "linux")]
pub mod container_path;
#[cfg(target_os = "windows")]
pub mod find_steam_dir;
pub mod find_workshop_folder;
pub mod get_workshop_item_path;
pub mod has_cdlc_installed;
#[cfg(all(target_os = "linux", feature = "client"))]
pub mod linux_launch_setup;
#[cfg(all(target_os = "linux", feature = "client"))]
pub mod linux_setup;
pub mod steam_install;
