use crate::application::pack::read::get_repo_info::GetRepoInfo;
use crate::application::pack::sync::config_sync_interactor::ConfigSyncInteractor;
use crate::application::pack::sync::sync_media::download_referenced_media;
use crate::application::pack::write::delete_pack::DeletePack;
use crate::application::pack::write::update_pack::UpdatePack;
use crate::application::pack::write::update_repo_config::UpdateRepoConfig;
use crate::application::repo::client::ClientRepoHandle;
use crate::domain::pack::pack_config::PackConfig;
use crate::domain::repo::repo_config::RepoConfig;
use crate::infrastructure::fs::files::file_paths::keyed_path::KeyedFilePath;
use crate::infrastructure::http::downloadable::KnownDownloadable;
use crate::infrastructure::http::remote_version::verify_remote_version;
use anyhow::{Context, anyhow, ensure};
use log::debug;

impl ClientRepoHandle {
    pub async fn sync_repo_config(
        &mut self,
        interactor: &impl ConfigSyncInteractor,
    ) -> anyhow::Result<()> {
        let remote_url = self.get_remote_url().clone();

        verify_remote_version(&remote_url).await?;

        let remote_repo_config = RepoConfig::download_known(&remote_url)
            .await
            .context(anyhow!(
                "Failed to download remote config from {}",
                remote_url
            ))?;

        // A local pack owns its folder outright. If the remote starts publishing a
        // pack of the same name, the `added` loop below would lay a downloaded
        // pack straight over it, so refuse before anything has been applied.
        let collisions = remote_repo_config
            .packs
            .iter()
            .filter(|p| self.is_local_pack(p))
            .collect::<Vec<_>>();
        ensure!(
            collisions.is_empty(),
            "Remote repository introduced pack(s) {:?} whose name(s) collide with existing \
             local pack(s). Rename or remove the local pack(s), then sync again.",
            collisions
        );

        let local_repo_config = self.get_config().clone();

        let removed = local_repo_config
            .packs
            .iter()
            .filter(|p| !remote_repo_config.packs.contains(*p))
            .collect::<Vec<_>>();

        for pack in removed {
            debug!("Pack '{}' has been removed from remote repository.", pack);
            let outcome = interactor.confirm_pack_removal(pack)?;
            self.delete_pack(pack, outcome).await?;
        }

        let added = remote_repo_config
            .packs
            .iter()
            .filter(|p| !local_repo_config.packs.contains(*p))
            .collect::<Vec<_>>();

        let mut pack_configs = Vec::with_capacity(remote_repo_config.packs.len());

        for pack in added {
            let path = PackConfig::file_path(pack);
            let pack_config = PackConfig::download_known(&path.with_base_url(&remote_url))
                .await
                .context(format!("Failed to download pack {} configuration", pack))?;

            self.add_pack(&pack_config).await?;

            interactor.notify_pack_added(pack)?;
            pack_configs.push(pack_config);
        }

        let existing = remote_repo_config
            .packs
            .iter()
            .filter(|p| local_repo_config.packs.contains(*p))
            .collect::<Vec<_>>();

        for pack in existing {
            let path = PackConfig::file_path(pack);
            let remote_pack_config = PackConfig::download_known(&path.with_base_url(&remote_url))
                .await
                .context(format!("Failed to download pack {} configuration", pack))?;
            self.update_pack(&remote_pack_config).await?;
            pack_configs.push(remote_pack_config);
        }

        self.update_repo_config(remote_repo_config.clone()).await?;

        download_referenced_media(
            self.get_repo_path(),
            &remote_url,
            &remote_repo_config,
            &pack_configs,
        )
        .await;

        Ok(())
    }
}
