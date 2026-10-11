use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_pack::GetPack;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_diff::PackDiff;

impl ClientRepoHandle {
    pub async fn get_pack_and_parents_diffs(
        &self,
        pack_name: &str,
        progress_reporter: impl ProgressReporter,
        force_refresh: bool,
    ) -> anyhow::Result<Vec<PackDiff>> {
        // Only the requested pack needs checking: a normal pack can never name a
        // local pack as its parent, since local packs exist on one machine only.
        self.ensure_not_local(pack_name)?;

        let chain = collect_pack_chain(self, pack_name).await?;

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

async fn collect_pack_chain(
    handle: &ClientRepoHandle,
    pack_name: &str,
) -> anyhow::Result<Vec<String>> {
    let mut chain = Vec::new();
    let mut current = pack_name.to_string();

    loop {
        let pack = handle.get_pack(&current).await?;
        let parent = pack.parent.clone();
        chain.push(current);

        match parent {
            Some(p) => current = p,
            None => break,
        }
    }

    chain.reverse();
    Ok(chain)
}
