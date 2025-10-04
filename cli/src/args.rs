use clap::{Parser, Subcommand};
use crate::commands::update_pack::UpdatePackArgs;

/// Pbo signing utility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: AppSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AppSubcommand {
    Init,
    Update(UpdatePackArgs),
}
