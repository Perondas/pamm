use crate::application::repo::client::ClientRepoHandle;
use crate::domain::pack::pack_diff::PackDiff;
use crate::infrastructure::progress::progress_reporter::ProgressReporter;
use anyhow::ensure;
use crate::support::collect_packs::collect_pack_names;

impl ClientRepoHandle {
    pub async fn get_pack_and_parents_diffs(
        &self,
        pack_name: &str,
        progress_reporter: impl ProgressReporter,
        force_refresh: bool,
    ) -> anyhow::Result<Vec<PackDiff>> {
        // Only the requested pack needs checking: a normal pack can never name a
        // local pack as its parent, since local packs exist on one machine only.
        ensure!(
            !self.is_local_pack(pack_name),
            "Pack '{}' is a local pack and cannot be diffed against the remote",
            pack_name
        );

        let chain = collect_pack_names(self, pack_name).await?;

        let mut diffs = Vec::with_capacity(chain.len());
        for name in chain {
            diffs.push(
                self.get_pack_diff(&name, progress_reporter.clone(), force_refresh)
                    .await?,
            );
        }
        Ok(diffs)
    }
}

