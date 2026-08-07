#[cfg(target_os = "linux")]
pub mod find_arma_install_dir;
#[cfg(target_os = "windows")]
pub mod find_steam_folder;
