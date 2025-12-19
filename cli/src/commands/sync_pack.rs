use crate::utils::diff_to_string::ToPrettyString;
use anyhow::anyhow;
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::fs::fs_readable::KnownFSReadable;
use pamm_lib::fs::fs_writable::NamedFSWritable;
use pamm_lib::manifest::pack_manifest::PackManifest;
use pamm_lib::net::apply_diff::apply_diff;
use pamm_lib::net::downloadable::{KnownDownloadable, NamedDownloadable};
use pamm_lib::repo::local_repo_config::LocalRepoConfig;
use pamm_lib::repo::repo_config::RepoConfig;
use std::env::current_dir;
use std::path::Path;

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

    sync_config(&current_dir, &local_repo_config)?;

    let remote_url = local_repo_config.get_remote();

    let remote_manifest = PackManifest::download_named(remote_url, &args.name)?;

    let diff = local_manifest.determine_pack_diff(&remote_manifest)?;

    if !diff.has_changes() {
        println!("Pack is already up to date.");
        return Ok(());
    }

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

fn sync_config(current_dir: &Path, local_repo_config: &LocalRepoConfig) -> anyhow::Result<()> {
    let remote_url = local_repo_config.get_remote();

    let remote_repo_config = RepoConfig::download_known(remote_url)?;

    let local_repo_config =
        RepoConfig::read_from_known(current_dir)?.ok_or(anyhow!("Local repo config not found"))?;

    todo!()
}
