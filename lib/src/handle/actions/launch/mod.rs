#[cfg(target_os = "windows")]
pub mod executable;
pub mod launch_pack;
#[cfg(target_os = "linux")]
mod preset;
#[cfg(target_os = "linux")]
pub mod setup;
#[cfg(not(target_os = "linux"))]
pub mod steam;
#[cfg(target_os = "linux")]
pub mod steam_linux;
