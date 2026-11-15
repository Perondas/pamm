use crate::domain::pack::pack_config::PackConfig;
use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_user_settings::RepoUserSettings;
use crate::domain::repo::repo_version::RepoVersion;
use crate::infrastructure::filesystem::files::file_paths::keyed_path::KeyedFilePath;
use crate::infrastructure::filesystem::files::fs_writable::FixedFsWritable;
use crate::infrastructure::filesystem::files::util::path_checks::is_dir;
use crate::infrastructure::http::downloadable::KnownDownloadable;
use crate::infrastructure::http::remote_version::verify_remote_version;
use anyhow::Context;
use std::path::{Path, PathBuf};
use tokio::fs;
use url::Url;

#[cfg(feature = "client")]
pub async fn init_client_repo_from_remote(
    parent_dir: &Path,
    remote_url: &Url,
) -> anyhow::Result<PathBuf> {
    if !is_dir(parent_dir).await {
        anyhow::bail!("{} is not a directory", parent_dir.display());
    }

    verify_remote_version(remote_url).await?;

    let repo = RepoConfig::download_known(remote_url)
        .await
        .context(format!(
            "Failed to download repo information from: {}",
            remote_url
        ))?;

    let base_path = parent_dir.join(&repo.name);

    if fs::try_exists(&base_path).await? {
        anyhow::bail!("Directory {} already exists", base_path.display());
    }

    fs::create_dir(&base_path).await?;
    repo.write_fixed(&base_path).await?;
    RepoVersion::current().write_fixed(&base_path).await?;

    let repo_user_settings = RepoUserSettings::new(remote_url.clone());
    repo_user_settings.write_fixed(&base_path).await?;

    let mut pack_configs = Vec::with_capacity(repo.packs.len());
    for pack in &repo.packs {
        let path = PackConfig::file_path(pack);
        let pack_config = PackConfig::download_known(&path.with_base_url(remote_url))
            .await
            .context(format!("Failed to download pack {} configuration", pack))?;

        pack_config.init_client_on_fs(&base_path).await?;
        pack_configs.push(pack_config);
    }

    crate::application::pack::sync::sync_media::download_referenced_media(
        &base_path,
        remote_url,
        &repo,
        &pack_configs,
    )
    .await;

    Ok(base_path)
}
