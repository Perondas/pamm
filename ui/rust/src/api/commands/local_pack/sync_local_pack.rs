use pamm_lib::application::repo::client::ClientRepoHandle;
use pamm_lib::infrastructure::progress::progress_reporter::NoopProgressReporter;
use std::path::Path;

pub async fn sync_local_pack(pack_name: String, repo_path: String) -> anyhow::Result<()> {
    let repo_dir = Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_dir).await?;

    handle
        .sync_local_pack(&pack_name, &NoopProgressReporter)
        .await?;

    Ok(())
}
