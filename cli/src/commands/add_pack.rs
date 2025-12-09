use crate::commands::input::from_cli_input::FromCliInputWithContext;
use clap::Args;
use pamm_lib::fs::fs_readable::KnownFSReadable;
use pamm_lib::fs::fs_writable::KnownFSWritable;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::repo::repo_config::RepoConfig;

#[derive(Debug, Args)]
pub struct AddPackArgs {}

pub fn add_pack_command(args: AddPackArgs) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let mut repo_config = RepoConfig::read_from_known(&current_dir)?.expect("Missing repo config");

    let pack_config = PackConfig::from_cli_input(&repo_config)?;

    repo_config.packs.push(pack_config.name.clone());

    repo_config.write_to_known(&current_dir)?;

    pack_config.init_blank_on_fs(&current_dir)?;

    Ok(())
}
