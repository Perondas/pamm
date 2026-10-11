//! Async counterparts of `Path::is_dir` / `Path::is_file`, which would
//! otherwise block the runtime on a filesystem stat.

use std::path::Path;
use tokio::fs;

pub(crate) async fn is_dir(path: impl AsRef<Path>) -> bool {
    fs::metadata(path).await.is_ok_and(|m| m.is_dir())
}

pub(crate) async fn is_file(path: impl AsRef<Path>) -> bool {
    fs::metadata(path).await.is_ok_and(|m| m.is_file())
}
