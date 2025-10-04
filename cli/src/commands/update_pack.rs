use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::consts::MANIFEST_FILE_NAME;
use pamm_lib::pack::pack_manifest::PackManifest;
use std::env::current_dir;
use std::fs;

#[derive(Debug, Args)]
pub struct UpdatePackArgs {
    /// Force refresh all addons, ignoring cached state
    #[arg(short, long, action)]
    pub force_refresh: bool,
}

pub fn update_pack_command(args: UpdatePackArgs) -> anyhow::Result<()> {
    let pack_file = current_dir()?.join(MANIFEST_FILE_NAME);

    let manifest = if pack_file.exists() {
        let file = fs::File::open(&pack_file)?;
        serde_cbor::from_reader(file)?
    } else {
        println!("No pack found in the current directory.");
        println!("Reinitializing a new pack manifest.");
        PackManifest::default()
    };

    let diff = manifest.determine_pack_diff(args.force_refresh)?;

    println!("Pack Update Summary:");
    println!("Added: {}", diff.added.len());
    println!("Removed: {}", diff.removed.len());
    println!("Changed: {}", diff.changed.len());

    if diff.added.is_empty() && diff.removed.is_empty() && diff.changed.is_empty() {
        println!("No changes detected. Your pack is up to date.");
        return Ok(());
    }

    // TODO: Show more detailed summary of changes

    let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to apply these changes?")
        .default(false)
        .interact()?;

    if !outcome {
        println!("Aborting update.");
        return Ok(());
    }

    manifest.apply_pack_diff(diff)?;

    Ok(())
}
