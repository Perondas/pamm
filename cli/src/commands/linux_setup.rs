use clap::Args;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use std::env::current_dir;

/// Print the one-time Steam setup this repo needs on Linux
#[derive(Debug, Args)]
pub struct SetupArgs {}

/// Prints the one-time Steam setup for this repo.
pub fn setup_command(_args: SetupArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?)?;
    let setup = handle.linux_launch_setup()?;

    println!("Steam install:  {:?}", setup.flavour);
    println!("Arma 3:         {}", setup.arma_install_dir);
    println!();

    match &setup.launch_options {
        Some(launch_options) => {
            println!("Paste this into Arma 3's Steam launch options (right-click Arma 3 →");
            println!("Properties → Launch Options). Steam can stay open.");
            println!();
            println!("    {launch_options}");
        }
        None => println!("Steam needs no launch options for this repo."),
    }

    if let Some(command) = &setup.flatpak_override_command {
        println!();
        println!("This repo's mods live outside the Flatpak sandbox, so Steam also needs");
        println!("access to them. Run this, then restart Steam:");
        println!();
        println!("    {command}");
    }

    println!();

    Ok(())
}
