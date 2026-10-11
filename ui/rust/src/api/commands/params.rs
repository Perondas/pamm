use pamm_lib::handle::client_repo_handle::ClientRepoHandle;

pub async fn get_launch_params(
    repot_path: String,
    pack_name: String,
) -> anyhow::Result<Vec<String>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = ClientRepoHandle::open(repot_path).await?;

    handle.get_pack_launch_params(&pack_name).await
}

pub async fn get_server_launch_params(
    repot_path: String,
    pack_name: String,
) -> anyhow::Result<Vec<String>> {
    let repot_path = std::path::Path::new(&repot_path);

    let handle = ClientRepoHandle::open(repot_path).await?;

    handle.get_pack_server_launch_params(&pack_name).await
}

pub async fn set_launch_params(
    repot_path: String,
    pack_name: String,
    launch_params: Vec<String>,
) -> anyhow::Result<()> {
    let repot_path = std::path::Path::new(&repot_path);
    let handle = ClientRepoHandle::open(repot_path).await?;

    handle
        .set_pack_launch_params(&pack_name, launch_params)
        .await
}
