pub mod build_pack;
pub mod build_repo;
pub mod materializer;

#[cfg(test)]
mod tests;

use crate::handle::actions::build::materializer::Materializer;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BuildMode {
    #[default]
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

#[derive(Debug, Default)]
pub struct BuildReport {
    pub files_materialized: usize,
    pub stale_removed: usize,
    pub mode: BuildMode,
}

impl<'a> From<&Materializer<'a>> for BuildReport {
    fn from(m: &Materializer) -> Self {
        Self {
            mode: m.mode,
            ..Self::default()
        }
    }
}

impl Add for BuildReport {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.mode != rhs.mode {
            panic!("Can only add PackBuildReports with the same pack name and mode");
        }

        Self {
            files_materialized: self.files_materialized + rhs.files_materialized,
            stale_removed: self.stale_removed + rhs.stale_removed,
            mode: self.mode,
        }
    }
}
