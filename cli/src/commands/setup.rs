use clap::Args;

/// Print the one-time Steam setup this repo needs on Linux
#[derive(Debug, Args)]
pub struct SetupArgs {}

/// Prints the one-time Steam setup for this repo.
///
/// pamm deliberately does not apply any of this: it never writes Steam's
/// configuration and never runs `flatpak`. The user pastes and runs these
/// themselves, with Steam running — there is no close-Steam step.
#[cfg(target_os = "linux")]
pub fn setup_command(_args: SetupArgs) -> anyhow::Result<()> {
    use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
    use std::env::current_dir;

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

/// The setup this reports is entirely about getting mod directories across the
/// Proton container and Flatpak sandbox boundaries, neither of which exists off
/// Linux. Reporting that plainly beats erroring on a command the user can
/// reasonably try.
#[cfg(not(target_os = "linux"))]
pub fn setup_command(_args: SetupArgs) -> anyhow::Result<()> {
    println!("No setup is needed on this platform — Arma is launched directly.");

    Ok(())
}
