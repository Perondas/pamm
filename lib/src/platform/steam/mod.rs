pub const ARMA_APP_ID: u32 = 107_410;

#[cfg(target_os = "linux")]
pub mod arma_install;
#[cfg(target_os = "linux")]
pub mod container_path;
#[cfg(target_os = "windows")]
pub mod find_steam_dir;
pub mod find_workshop_folder;
#[cfg(all(target_os = "linux", feature = "client"))]
pub mod linux_launch_setup;
#[cfg(all(target_os = "linux", feature = "client"))]
pub mod linux_setup;
#[cfg(target_os = "linux")]
pub mod steam_install;
