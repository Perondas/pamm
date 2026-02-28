use crate::actions::sync::interactor::ConfigSyncInteractor;
use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::fs_writable::IdentifiableFSWritable;
use crate::io::fs::fs_writable::KnownFSWritable;
use crate::io::fs::pack::delete_pack::delete_pack;
use crate::io::net::downloadable::{KnownDownloadable, NamedDownloadable};
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use anyhow::{anyhow, Context};
use log::debug;
use std::path::Path;

pub fn sync_pack_config(
    pack_dir: &Path,
    interactor: &impl ConfigSyncInteractor,
) -> anyhow::Result<RepoConfig> {
    let repo_user_settings = RepoUserSettings::read_from_known(pack_dir)
        .context(anyhow!("Failed to read settings file in {:#?}", pack_dir))?;

    let remote_url = repo_user_settings.get_remote();

    let remote_repo_config = RepoConfig::download_known(remote_url).context(anyhow!(
        "Failed to download remote config from {}",
        remote_url
    ))?;

    let local_repo_config = RepoConfig::read_from_known(pack_dir)
        .context(anyhow!("Failed to read repo config in {:#?}", pack_dir))?;

    let removed = local_repo_config
        .packs
        .iter()
        .filter(|p| !remote_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in removed {
        debug!("Pack '{}' has been removed from remote repository.", pack);
        let outcome = interactor.confirm_pack_removal(pack)?;
        if outcome {
            delete_pack(pack_dir, pack)?;
            debug!("Pack '{}' removed locally.", pack);
        }
    }

    let added = remote_repo_config
        .packs
        .iter()
        .filter(|p| !local_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in added {
        let pack_config = PackConfig::download_named(remote_url, pack)
            .context(format!("Failed to download pack {} configuration", &pack))?;

        pack_config.init_blank_on_fs(pack_dir)?;

        interactor.notify_pack_added(pack)?;
    }

    let existing = remote_repo_config
        .packs
        .iter()
        .filter(|p| local_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in existing {
        let remote_pack_config = PackConfig::download_named(remote_url, pack)
            .context(format!("Failed to download pack {} configuration", &pack))?;
        remote_pack_config.write_to(pack_dir)?;
    }

    remote_repo_config.write_to(pack_dir)?;

    Ok(remote_repo_config)
}
