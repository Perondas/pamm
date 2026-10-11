use clap::Args;

/// Open a file browser inside Arma's Proton prefix
#[derive(Debug, Args)]
pub struct ExplorerArgs {
    /// Windows path to start in, e.g. `C:\` or `Z:\home`
    #[arg()]
    pub path: Option<String>,
}
