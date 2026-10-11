use crate::subcommands::linux::proton::detect_proton;
use clap::Args;

/// Open a file browser inside Arma's Proton prefix
#[derive(Debug, Args)]
pub struct ExplorerArgs {
    /// Windows path to start in, e.g. `C:\` or `Z:\home`
    #[arg()]
    pub path: Option<String>,
}

pub fn explorer_command(args: ExplorerArgs) -> anyhow::Result<()> {
    detect_proton()?.open_explorer(args.path.as_deref())?;

    Ok(())
}
