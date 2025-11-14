use crate::commands::input::from_cli_input::FromCliInput;
use pamm_lib::pack::pack_config::PackConfig;

pub fn init_pack_command() -> anyhow::Result<()> {
    let pack_config = PackConfig::from_cli_input()?;

    pack_config.init_on_disk(&std::env::current_dir()?)?;

    Ok(())
}
