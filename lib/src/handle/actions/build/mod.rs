pub mod build_pack;
pub mod build_repo;
pub mod materializer;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildMode {
    Symlink,
    Copy,
}

#[derive(Debug, Clone, Copy)]
pub struct BuildOptions {
    pub mode: BuildMode,
    pub force_refresh: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            mode: BuildMode::Symlink,
            force_refresh: false,
        }
    }
}

#[derive(Debug)]
pub struct PackBuildReport {
    pub pack_name: String,
    pub addons_materialized: usize,
    pub files_materialized: usize,
    pub stale_removed: usize,
    pub mode_used: BuildMode,
}

#[derive(Debug)]
pub struct BuildReport {
    pub packs: Vec<PackBuildReport>,
}
