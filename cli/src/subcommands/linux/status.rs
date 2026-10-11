use crate::subcommands::linux::proton::detect_proton;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::util::proton::teamspeak::find_teamspeak;
use std::env::current_dir;

/// Prints the one-time Steam setup for this repo, and the state of Arma's
/// Proton prefix.
pub fn status_command() -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&current_dir()?)?;
    let setup = handle.linux_launch_setup()?;

    println!("Steam install:  {:?}", setup.flavour);
    println!("Arma 3:         {}", setup.arma_install_dir);
    print_proton_status();
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

    println!("Use the help subcommand for more information on other commands.");

    Ok(())
}

fn print_proton_status() {
    let proton = match detect_proton() {
        Ok(proton) => proton,
        Err(e) => {
            println!("Proton:         unavailable ({e:#})");
            return;
        }
    };

    match proton.version() {
        Ok(version) => println!(
            "Proton:         {version} ({})",
            proton.proton_dir.display()
        ),
        Err(_) => println!("Proton:         {}", proton.proton_dir.display()),
    }

    match find_teamspeak(&proton.prefix()) {
        Some(exe) => println!("TeamSpeak:      installed ({})", exe.display()),
        None => println!("TeamSpeak:      not installed"),
    }
}
