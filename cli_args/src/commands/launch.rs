use clap::{Args, ValueEnum};

/// Launch a pack by name
#[derive(Debug, Args)]
pub struct LaunchArgs {
    /// Pack name
    #[arg()]
    pub name: String,

    /// How to start the game
    #[arg(long, value_enum, default_value_t = LaunchMode::Steam)]
    pub launch_type: LaunchMode,

    /// Launch without any optional addons enabled
    #[arg(long)]
    pub no_optionals: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[non_exhaustive]
pub enum LaunchMode {
    /// Launch through Steam
    Steam,
    /// Launch the game executable directly
    #[cfg(target_os = "windows")]
    File,
}
