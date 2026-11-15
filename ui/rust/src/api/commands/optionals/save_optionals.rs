use pamm_lib::application::addons::optionals::optional_addon::OptionalAddon;
use pamm_lib::application::addons::optionals::SaveOptionals;
use pamm_lib::application::repo::client::ClientRepoHandle;

pub async fn save_optionals(
    repo_path: String,
    pack_name: String,
    optionals: Vec<OptionalAddon>,
) -> anyhow::Result<()> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path).await?;

    handle.save_optionals(&pack_name, &optionals).await
}
