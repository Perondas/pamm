#[cfg(target_os = "linux")]
pub mod arma_install;
#[cfg(target_os = "windows")]
pub mod find_steam_dir;
#[cfg(target_os = "linux")]
pub mod flatpak;
#[cfg(target_os = "linux")]
pub mod steam_install;
