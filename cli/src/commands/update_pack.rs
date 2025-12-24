use crate::utils::diff_to_string::ToPrettyString;
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::io::fs::fs_readable::NamedFSReadable;
use pamm_lib::io::fs::fs_writable::NamedFSWritable;
use pamm_lib::io::fs::pack::index_generator::IndexGenerator;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_diff::diff_packs;
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

    let config =
        PackConfig::read_from_named(&current_dir, &args.name)?.expect("Missing pack config");

    let stored_index = config.read_index_from_fs(&current_dir)?;

    let index_generator = IndexGenerator::from_config(&config, &current_dir)?;

    let actual_index = index_generator.index_addons()?;

    let diff = diff_packs(stored_index, actual_index.clone())?;

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

    diff.write_index_to_fs(&current_dir, &actual_index)?;

    let config = config.with_addons(actual_index.to_map());

    config.write_to_named(&current_dir, &config.name)?;

    Ok(())
}
