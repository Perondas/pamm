use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::consts::MANIFEST_FILE_NAME;
use pamm_lib::pack::pack_manifest::PackManifest;
use pamm_lib::serialization::{from_reader, to_writer};
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

    let stored_manifest = if pack_file.exists() {
        let mut file = fs::File::open(&pack_file)?;
        from_reader(&mut file)?
    } else {
        println!("No pack found in the current directory.");
        println!("Reinitializing a new pack manifest.");
        PackManifest::default()
    };

    let fs_manifest = PackManifest::load_from_fs(&current_dir()?, args.force_refresh)?;

    let diff = stored_manifest.determine_pack_diff(&fs_manifest)?;

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

    let mut file = fs::File::create(&pack_file)?;
    to_writer(&mut file, &fs_manifest)?;

    Ok(())
}
