use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::files::name_consts::MP_MISSIONS_DIR_NAME;
use anyhow::{Context, anyhow};
use log::trace;
use std::path::PathBuf;
use tokio::fs;

impl<T> ResolveMissions for T
where
    T: GetPack + GetRepoInfo + Sync,
{
    async fn resolve_missions(&self, pack_name: &str) -> anyhow::Result<Vec<PathBuf>> {
        let mut missions = Vec::new();
        let mut current = Some(pack_name.to_string());

        // Own addons first, then each ancestor's in turn.
        while let Some(pack_name) = current {
            let pack_config = self.get_pack(&pack_name).await?;

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

            current = pack_config.parent;
        }

        Ok(missions)
    }
}

pub(in crate::handle) trait ResolveMissions {
    /// Gets the paths to the pack's missions (including inherited ones).
    fn resolve_missions(
        &self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<Vec<PathBuf>>> + Send;
}
