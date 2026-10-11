use pamm_cli_args::commands::launch::{LaunchArgs, LaunchMode};
use pamm_lib::handle::actions::launch::launch_pack::LaunchParams;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::env::current_dir;

fn to_lib_mode(mode: LaunchMode) -> pamm_lib::handle::actions::launch::launch_pack::LaunchMode {
    use pamm_lib::handle::actions::launch::launch_pack::LaunchMode as LibLaunchMode;

    match mode {
        #[cfg(target_os = "windows")]
        LaunchMode::File => LibLaunchMode::Executable,
        _ => LibLaunchMode::Steam,
    }
}

pub async fn launch_command(args: LaunchArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?).await?;
    let launch_params = LaunchParams::new(to_lib_mode(args.launch_type), args.no_optionals);
    let outcome = handle.launch_pack(&args.name, &launch_params).await?;

    if let Some(preset_path) = outcome.preset_path {
        println!("Wrote preset to {preset_path}");
        println!("If Arma starts without mods, check `pamm linux`.");
    }

    Ok(())
}
