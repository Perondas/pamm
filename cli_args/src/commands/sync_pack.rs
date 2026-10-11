use clap::Args;

/// Sync your local repo state with the remote repository
#[derive(Debug, Args)]
pub struct SyncPackArgs {
    #[arg()]
    /// Name of the pack to sync
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    /// Discards indices, reads all addons from disk.
    /// Does nothing for local packs
    pub force_refresh: bool,
    #[arg(short, long)]
    /// Assume "yes" as the answer to all prompts
    pub yes: bool,
    /// Silent mode, minimal output
    #[arg(short, long, action)]
    pub silent: bool,
}
