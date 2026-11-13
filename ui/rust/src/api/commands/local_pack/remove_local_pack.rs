use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
pub async fn remove_local_pack(repo_path: String, pack_name: String) -> anyhow::Result<()> {
    let repot_path = std::path::Path::new(&repo_path);

    let mut handle: ClientRepoHandle = pamm_lib::handle::repo_handle::RepoHandle::open(repot_path)
        .await?
        .try_into()?;

    handle.remove_local_pack(&pack_name).await
}
