use clap::Args;

/// Build a pack or the entire repository
#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Pack name; if omitted, builds the entire repo.
    #[arg()]
    pub name: Option<String>,

    /// Materialize files in `www/` as copies instead of symlinks.
    #[arg(long, action, conflicts_with = "symlink")]
    pub copy: bool,

    /// Materialize files in `www/` as symlinks (default).
    #[arg(long, action)]
    pub symlink: bool,

    /// Re-index everything from scratch, ignoring the cache.
    #[arg(short, long, action)]
    pub force_refresh: bool,

    /// Silent mode, minimal output.
    #[arg(short, long, action)]
    pub silent: bool,
}
