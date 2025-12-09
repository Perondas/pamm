use crate::fs::fs_writable::{KnownFSWritable, NamedFSWritable};
use crate::net::downloadable::{KnownDownloadable, NamedDownloadable};
use crate::pack::pack_config::PackConfig;
use crate::repo::local_repo_config::LocalRepoConfig;
use crate::repo::repo_config::RepoConfig;
use anyhow::Context;
use std::fs;
use std::path::Path;
use url::Url;

impl RepoConfig {
    pub fn init_blank_on_fs(&self, parent_dir: &Path) -> anyhow::Result<()> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let base_path = parent_dir.join(&self.name);

        fs::create_dir(&base_path)?;
        self.write_to_known(&base_path)?;

        Ok(())
    }

    pub fn init_from_remote(parent_dir: &Path, remote_url: &Url) -> anyhow::Result<Self> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        let repo = RepoConfig::download_known(remote_url).context(format!(
            "Failed to download repo information from: {}",
            remote_url
        ))?;

        let base_path = parent_dir.join(&repo.name);

        if base_path.exists() {
            anyhow::bail!("Directory {} already exists", base_path.display());
        }

        fs::create_dir(&base_path)?;
        repo.write_to_known(&base_path)?;

        let local_config = LocalRepoConfig::new(remote_url.clone());
        local_config.write_to_known(&base_path)?;

        for pack in &repo.packs {
            let pack_config = PackConfig::download_named(remote_url, pack)
                .context(format!("Failed to download pack {} configuration", &pack))?;

            pack_config.init_blank_on_fs(&base_path)?;
        }

        Ok(repo)
    }
}
