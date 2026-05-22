use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::optionals::optional_addon::OptionalAddon;
use pamm_lib::handle::optionals::SaveOptionals;

pub fn save_optionals(
    repo_path: String,
    pack_name: String,
    optionals: Vec<OptionalAddon>,
) -> anyhow::Result<()> {
    let repo_path = std::path::Path::new(&repo_path);

    let handle = ClientRepoHandle::open(repo_path)?;

    handle.save_optionals(&pack_name, &optionals)
}
