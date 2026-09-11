use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::io::progress_reporting::progress_reporter::NoopProgressReporter;
use std::path::Path;

pub fn sync_local_pack(pack_name: String, repo_path: String) -> anyhow::Result<()> {
    let repo_dir = Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_dir)?;

    handle
        .is_local_pack(&pack_name)
        .ok_or(anyhow::anyhow!("Pack '{}' is not a local pack.", pack_name))?;

    handle.sync_local_pack(&pack_name, &NoopProgressReporter)?;

    Ok(())
}
