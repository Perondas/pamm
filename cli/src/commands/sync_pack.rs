use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::consts::{CONFIG_FILE_NAME, MANIFEST_FILE_NAME};
use pamm_lib::dl::apply_diff::apply_diff;
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_manifest::PackManifest;
use std::env::current_dir;
use std::fs;

#[derive(Debug, Args)]
pub struct SyncPackArgs {}

pub fn sync_pack_command(_: SyncPackArgs) -> anyhow::Result<()> {
    let config_file = current_dir()?.join(CONFIG_FILE_NAME);

    let local_config: PackConfig = if config_file.exists() {
        let file = fs::File::open(&config_file)?;
        serde_json::from_reader(file)?
    } else {
        return Err(anyhow::anyhow!("config file does not exist"));
    };

    let local_manifest = PackManifest::gen_from_fs(&current_dir()?, false)?;

    // TODO: add remote config sync

    let remote_manifest_url = local_config
        .remote
        .expect("Remote not set in config")
        .join(MANIFEST_FILE_NAME)?;

    let remote_manifest = bincode::serde::decode_from_std_read::<PackManifest, _, _>(
        &mut ureq::get(remote_manifest_url.to_string())
            .call()?
            .body_mut()
            .as_reader(),
        bincode::config::standard(),
    )?;

    let diff = local_manifest.determine_pack_diff(&remote_manifest)?;

    if !diff.has_changes() {
        println!("Pack is already up to date.");
        return Ok(());
    }

    println!("The following changes were detected:");
    println!("{}", diff.to_pretty_string());

    let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to apply these changes?")
        .default(false)
        .interact()?;

    if !outcome {
        println!("Aborting sync.");
        return Ok(());
    }

    apply_diff(&current_dir()?, diff, &remote_manifest_url)?;

    let fs_manifest = PackManifest::gen_from_fs(&current_dir()?, false)?;

    let diff_after_patch = fs_manifest.determine_pack_diff(&remote_manifest)?;

    if diff_after_patch.has_changes() {
        return Err(anyhow::anyhow!(
            "Pack is still out of date after applying diff"
        ));
    }

    println!("Pack synchronized successfully.");

    Ok(())
}
