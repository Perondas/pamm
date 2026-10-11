use crate::subcommands::linux::proton::detect_proton;
use pamm_cli_args::subcommands::linux::kill::KillArgs;

pub fn kill_command(_args: KillArgs) -> anyhow::Result<()> {
    detect_proton()?.kill_all()
}
