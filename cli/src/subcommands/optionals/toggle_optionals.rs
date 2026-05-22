use clap::Args;
use pamm_lib::handle::client::optionals::SaveOptionals;
use pamm_lib::handle::client::optionals::load_optionals::LoadOptionals;
use pamm_lib::handle::client::client_repo_handle::ClientRepoHandle;

#[derive(Debug, Args)]
pub struct ToggleOptionalsArgs {
    /// Pack name
    #[arg()]
    pub name: String,
}

pub fn toggle_optionals_command(args: ToggleOptionalsArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?)?;

    let mut optionals = handle.load_optionals(&args.name)?;

    let selection = dialoguer::MultiSelect::new()
        .with_prompt("What optionals to enable?")
        .items(optionals.iter().map(|o| o.name.to_owned()))
        .defaults(&optionals.iter().map(|e| e.enabled).collect::<Vec<_>>())
        .interact()?;

    optionals.iter_mut().enumerate().for_each(|(i, e)| {
        e.enabled = selection.contains(&i);
    });

    handle.save_optionals(&args.name, &optionals)
}
