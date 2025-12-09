use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::fs::fs_readable::KnownFSReadable;
use pamm_lib::fs::fs_writable::NamedFSWritable;
use pamm_lib::manifest::pack_manifest::PackManifest;
use pamm_lib::net::apply_diff::apply_diff;
use pamm_lib::net::downloadable::NamedDownloadable;
use pamm_lib::repo::local_repo_config::LocalRepoConfig;
use std::env::current_dir;

#[derive(Debug, Args)]
pub struct SyncPackArgs {
    #[arg()]
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}

pub fn sync_pack_command(args: SyncPackArgs) -> anyhow::Result<()> {
    let current_dir = current_dir()?;

    let local_repo_config = LocalRepoConfig::read_from_known(&current_dir)?
        .expect("No remote config found in current directory");

    let local_manifest = PackManifest::gen_from_fs(&current_dir, &args.name, args.force)?;

    let remote_url = local_repo_config.get_remote();

    let remote_manifest = PackManifest::download_named(remote_url, &args.name)?;

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

    apply_diff(&current_dir, &args.name, diff, remote_url)?;

    let fs_manifest = PackManifest::gen_from_fs(&current_dir, &args.name, false)?;

    fs_manifest.write_to_named(&current_dir, &args.name)?;

    let diff_after_patch = fs_manifest.determine_pack_diff(&remote_manifest)?;

    if diff_after_patch.has_changes() {
        return Err(anyhow::anyhow!(
            "Pack is still out of date after applying diff"
        ));
    }

    println!("Pack synchronized successfully.");

    Ok(())
}
