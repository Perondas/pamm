use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::optionals::optional_addon::OptionalAddon;
use pamm_lib::handle::optionals::SaveOptionals;

pub async fn save_optionals(
    repo_path: String,
    pack_name: String,
    optionals: Vec<OptionalAddon>,
) -> anyhow::Result<()> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle: ClientRepoHandle = pamm_lib::handle::repo_handle::RepoHandle::open(repo_path)
        .await?
        .try_into()?;

    handle.save_optionals(&pack_name, &optionals).await
}
