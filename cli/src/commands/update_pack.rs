use crate::utils::diff_to_string::ToPrettyString;
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::fs::fs_readable::NamedFSReadable;
use pamm_lib::fs::fs_writable::NamedFSWritable;
use pamm_lib::manifest::pack_manifest::PackManifest;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct UpdatePackArgs {
    #[arg()]
    pub name: String,

    /// Force refresh all addons, ignoring cached state
    #[arg(short, long, action)]
    pub force_refresh: bool,
}

pub fn update_pack_command(args: UpdatePackArgs) -> anyhow::Result<()> {
    let current_dir = current_dir()?;

    let stored_manifest =
        PackManifest::read_from_named(&current_dir, &args.name)?.unwrap_or_default();

    let fs_manifest = PackManifest::gen_from_fs(&current_dir, &args.name, args.force_refresh)?;

    let diff = stored_manifest.determine_pack_diff(&fs_manifest)?;

    if !diff.has_changes() {
        println!("No changes found");
        return Ok(());
    }

    println!("{}", diff.to_pretty_string());

    let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to apply these changes?")
        .default(false)
        .interact()?;

    if !outcome {
        println!("Aborting update.");
        return Ok(());
    }

    fs_manifest.write_to_named(&current_dir, &args.name)?;

    Ok(())
}
