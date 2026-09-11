use std::fs;
use std::path::Path;
use anyhow::Context;
use url::Url;
use crate::io::files::file_paths::keyed_path::KeyedFilePath;
use crate::io::fs::fs_writable::FixedFsWritable;
use crate::io::net::downloadable::KnownDownloadable;
use crate::io::net::remote_version::verify_remote_version;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use crate::models::repo::repo_version::RepoVersion;

impl RepoConfig {
    #[cfg(feature = "client")]
    pub fn init_from_remote(
        parent_dir: &Path,
        remote_url: &Url,
    ) -> anyhow::Result<(Self, RepoUserSettings)> {
        if !parent_dir.is_dir() {
            anyhow::bail!("{} is not a directory", parent_dir.display());
        }

        verify_remote_version(remote_url)?;

        let repo = RepoConfig::download_known(remote_url).context(format!(
            "Failed to download repo information from: {}",
            remote_url
        ))?;

        let base_path = parent_dir.join(&repo.name);

        if base_path.exists() {
            anyhow::bail!("Directory {} already exists", base_path.display());
        }

        fs::create_dir(&base_path)?;
        repo.write_fixed(&base_path)?;
        RepoVersion::current().write_fixed(&base_path)?;

        let repo_user_settings = RepoUserSettings::new(remote_url.clone());
        repo_user_settings.write_fixed(&base_path)?;

        let mut pack_configs = Vec::with_capacity(repo.packs.len());
        for pack in &repo.packs {
            let path = PackConfig::file_path(pack);
            let pack_config = PackConfig::download_known(&path.with_base_url(remote_url))
                .context(format!("Failed to download pack {} configuration", pack))?;

            pack_config.init_client_on_fs(&base_path)?;
            pack_configs.push(pack_config);
        }

        crate::handle::actions::sync::sync_media::download_referenced_media(
            &base_path,
            remote_url,
            &repo,
            &pack_configs,
        );

        Ok((repo, repo_user_settings))
    }
}