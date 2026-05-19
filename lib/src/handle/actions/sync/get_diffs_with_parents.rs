use crate::handle::reading::get_pack::GetPack;
use crate::handle::repo_handle::RepoHandle;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_diff::PackDiff;

impl RepoHandle {
    pub fn get_pack_and_parents_diffs<P: ProgressReporter>(
        &self,
        pack_name: &str,
        progress_reporter: P,
        force_refresh: bool,
    ) -> anyhow::Result<Vec<PackDiff>> {
        let chain = collect_pack_chain(self, pack_name)?;

        chain
            .into_iter()
            .map(|name| self.get_pack_diff(&name, progress_reporter.clone(), force_refresh))
            .collect()
    }
}

fn collect_pack_chain(handle: &RepoHandle, pack_name: &str) -> anyhow::Result<Vec<String>> {
    let mut chain = Vec::new();
    let mut current = pack_name.to_string();

    loop {
        let pack = handle.get_pack(&current)?;
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
