use crate::application::pack::read::get_repo_info::GetRepoInfo;
use crate::application::repo::server::ServerRepoHandle;
use crate::infrastructure::fs::files::name_consts::MP_MISSIONS_DIR_NAME;
use crate::support::collect_packs::collect_pack_names;
use anyhow::{Context, anyhow};
use log::trace;
use std::path::PathBuf;
use tokio::fs;

pub(in crate::application) trait ResolveMissions {
    /// Gets the paths to the pack's missions (including inherited ones).
    fn resolve_missions(
        &self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<Vec<PathBuf>>> + Send;
}

impl ResolveMissions for ServerRepoHandle {
    async fn resolve_missions(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>> {
        let mut missions = Vec::new();
        let pack_names = collect_pack_names(self, pack_name).await?;

        for pack_name in pack_names {
            let missions_dir = self
                .get_repo_path()
                .join(&pack_name)
                .join(MP_MISSIONS_DIR_NAME);

            let mut entries = fs::read_dir(&missions_dir).await.context(anyhow!(
                "Failed to read server missions folder at {:?}",
                missions_dir
            ))?;

            while let Some(entry) = entries.next_entry().await.context(anyhow!(
                "Failed to read server missions folder at {:?}",
                missions_dir
            ))? {
                let path = entry.path();
                if path.is_file() {
                    trace!("Found mission file: {:?}", path);
                    missions.push(path);
                }
            }
        }

        Ok(missions)
    }
}
