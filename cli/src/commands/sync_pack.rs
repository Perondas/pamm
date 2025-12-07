use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::fs::fs_readable::KnownFSReadable;
use pamm_lib::fs::fs_writable::KnownFSWritable;
use pamm_lib::net::apply_diff::apply_diff;
use pamm_lib::net::downloadable::KnownDownloadable;
use pamm_lib::pack::manifest::pack_manifest::PackManifest;
use pamm_lib::repo::remote_config::RemoteConfig;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct SyncPackArgs {}

pub fn sync_pack_command(_: SyncPackArgs) -> anyhow::Result<()> {
    let current_dir = current_dir()?;

    // TODO: make refresh optional
    let local_manifest = PackManifest::gen_from_fs(&current_dir, false)?;

    // TODO: add remote config sync

    let remote_config = RemoteConfig::read_from_known(&current_dir)?
        .expect("No remote config found in current directory");

    let remote_url = remote_config.get_remote();

    let remote_manifest = PackManifest::download_known(remote_url)?;

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

    apply_diff(&current_dir, diff, remote_url)?;

    let fs_manifest = PackManifest::gen_from_fs(&current_dir, false)?;

    fs_manifest.write_to_known(&current_dir)?;

    let diff_after_patch = fs_manifest.determine_pack_diff(&remote_manifest)?;

    if diff_after_patch.has_changes() {
        return Err(anyhow::anyhow!(
            "Pack is still out of date after applying diff"
        ));
    }

    println!("Pack synchronized successfully.");

    Ok(())
}
