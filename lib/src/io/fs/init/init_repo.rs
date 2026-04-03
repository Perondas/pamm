use crate::io::fs::fs_writable::KnownFSWritable;
use crate::io::net::downloadable::{KnownDownloadable, NamedDownloadable};
use crate::migration::version_tag::VersionTag;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

impl RepoConfig {
    pub fn init_blank_on_fs(&self, dest_dir: &Path) -> anyhow::Result<PathBuf> {
        if !dest_dir.is_dir() {
            anyhow::bail!("{} is not a directory", dest_dir.display());
        }

        let base_path = dest_dir.join(&self.name);

        if base_path.exists() {
            anyhow::bail!("Directory {} already exists", base_path.display());
        }

        fs::create_dir(&base_path)?;
        self.write_to(&base_path)?;

        VersionTag::get_latest().write_to(&base_path)?;

        Ok(base_path)
    }

    pub fn init_from_remote(
        parent_dir: &Path,
        remote_url: &Url,
    ) -> anyhow::Result<(Self, RepoUserSettings)> {
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
        repo.write_to(&base_path)?;

        let repo_user_settings = RepoUserSettings::new(remote_url.clone());
        repo_user_settings.write_to(&base_path)?;

        for pack in &repo.packs {
            let pack_config = PackConfig::download_named(remote_url, pack)
                .context(format!("Failed to download pack {} configuration", &pack))?;

            pack_config.init_blank_on_fs(&base_path)?;
        }

        VersionTag::get_latest().write_to(&base_path)?;

        Ok((repo, repo_user_settings))
    }
}
