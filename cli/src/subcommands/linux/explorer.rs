use crate::subcommands::linux::proton::detect_proton;
use pamm_cli_args::subcommands::linux::explorer::ExplorerArgs;

pub fn explorer_command(args: ExplorerArgs) -> anyhow::Result<()> {
    detect_proton()?.open_explorer(args.path.as_deref())?;

    Ok(())
}
