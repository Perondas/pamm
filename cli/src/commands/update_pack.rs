use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::pack::pack_manifest::PackManifest;
use pamm_lib::serialization::to_writer;
use std::env::current_dir;
use std::fs;

#[derive(Debug, Args)]
pub struct UpdatePackArgs {
    /// Force refresh all addons, ignoring cached state
    #[arg(short, long, action)]
    pub force_refresh: bool,
}

pub fn update_pack_command(args: UpdatePackArgs) -> anyhow::Result<()> {
    let stored_manifest = PackManifest::read_or_gen(&std::env::current_dir()?)?;

    let generated_manifest = PackManifest::gen_from_fs(&current_dir()?, args.force_refresh)?;

    let diff = stored_manifest.determine_pack_diff(&generated_manifest)?;

    if !diff.has_changes() {
        println!("No changes found");
        return Ok(());
    }

    println!("The following changes were detected:");
    println!("{}", diff.to_pretty_string());

    let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to apply these changes?")
        .default(false)
        .interact()?;

    if !outcome {
        println!("Aborting update.");
        return Ok(());
    }

    generated_manifest.write_to_fs(&current_dir()?)?;

    Ok(())
}
