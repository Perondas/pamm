use pamm_lib::handle::client_repo_handle::ClientRepoHandle;

pub async fn remove_local_pack(repo_path: String, pack_name: String) -> anyhow::Result<()> {
    let repot_path = std::path::Path::new(&repo_path);

    let mut handle = ClientRepoHandle::open(repot_path).await?;

    handle.remove_local_pack(&pack_name).await
}
