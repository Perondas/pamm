use crate::commands::input::from_cli_input::FromCliInput;
use pamm_lib::fs::init_pack::init_pack_on_fs;
use pamm_lib::pack::config::pack_config::PackConfig;

pub fn init_pack_command() -> anyhow::Result<()> {
    let pack_config = PackConfig::from_cli_input()?;

    init_pack_on_fs(&pack_config, &std::env::current_dir()?)?;

    Ok(())
}
