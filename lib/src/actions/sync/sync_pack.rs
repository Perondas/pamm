use crate::actions::sync::interactor::ConfigSyncInteractor;
use crate::handle::repo_handle::RepoHandle;
use crate::io::net::downloadable::{KnownDownloadable, NamedDownloadable};
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use anyhow::{anyhow, Context};
use log::debug;

// TODO: make associated with RepoHandle
pub fn sync_pack_config(
    repo_handle: &mut RepoHandle,
    interactor: &impl ConfigSyncInteractor,
) -> anyhow::Result<()> {
    let repo_user_settings = repo_handle
        .repo_user_settings
        .as_ref()
        .ok_or_else(|| anyhow!("Repo user settings not found"))?;

    let remote_url = repo_user_settings.get_remote().clone();

    let remote_repo_config = RepoConfig::download_known(&remote_url).context(anyhow!(
        "Failed to download remote config from {}",
        remote_url
    ))?;

    let local_repo_config = repo_handle.get_config().clone();

    let removed = local_repo_config
        .packs
        .iter()
        .filter(|p| !remote_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in removed {
        debug!("Pack '{}' has been removed from remote repository.", pack);
        let outcome = interactor.confirm_pack_removal(pack)?;
        repo_handle.delete_pack(pack, outcome)?;
    }

    let added = remote_repo_config
        .packs
        .iter()
        .filter(|p| !local_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in added {
        let pack_config = PackConfig::download_named(&remote_url, pack)
            .context(format!("Failed to download pack {} configuration", &pack))?;

        repo_handle.add_pack(&pack_config)?;

        interactor.notify_pack_added(pack)?;
    }

    let existing = remote_repo_config
        .packs
        .iter()
        .filter(|p| local_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in existing {
        let remote_pack_config = PackConfig::download_named(&remote_url, pack)
            .context(format!("Failed to download pack {} configuration", &pack))?;
        repo_handle.update_pack(&remote_pack_config)?;
    }

    repo_handle.update_repo_config(remote_repo_config)?;

    Ok(())
}
