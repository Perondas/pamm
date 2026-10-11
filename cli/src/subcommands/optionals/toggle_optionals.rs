use pamm_cli_args::subcommands::optionals::toggle_optionals::ToggleOptionalsArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::optionals::SaveOptionals;
use pamm_lib::handle::optionals::load_optionals::LoadOptionals;

pub async fn toggle_optionals_command(args: ToggleOptionalsArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?).await?;

    let mut optionals = handle.load_optionals(&args.name).await?;

    let selection = dialoguer::MultiSelect::new()
        .with_prompt("What optionals to enable?")
        .items(optionals.iter().map(|o| o.name.to_owned()))
        .defaults(&optionals.iter().map(|e| e.enabled).collect::<Vec<_>>())
        .interact()?;

    optionals.iter_mut().enumerate().for_each(|(i, e)| {
        e.enabled = selection.contains(&i);
    });

    handle.save_optionals(&args.name, &optionals).await
}
