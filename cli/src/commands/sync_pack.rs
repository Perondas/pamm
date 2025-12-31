use crate::progress_reporting::IndicatifProgressReporter;
use crate::utils::diff_to_string::ToPrettyString;
use anyhow::{Context, anyhow};
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use pamm_lib::io::fs::fs_readable::{KnownFSReadable, NamedFSReadable};
use pamm_lib::io::fs::fs_writable::IdentifiableFSWritable;
use pamm_lib::io::fs::fs_writable::KnownFSWritable;
use pamm_lib::io::fs::pack::delete_pack::delete_pack;
use pamm_lib::io::fs::pack::index_generator::IndexGenerator;
use pamm_lib::io::net::downloadable::{KnownDownloadable, NamedDownloadable};
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::pack::pack_diff::diff_packs;
use pamm_lib::repo::repo_config::RepoConfig;
use pamm_lib::repo::repo_user_settings::RepoUserSettings;
use std::env::current_dir;
use std::path::Path;

#[derive(Debug, Args)]
pub struct SyncPackArgs {
    #[arg()]
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    pub force_refresh: bool,
    /// Silent mode, minimal output
    #[arg(short, long, action)]
    pub silent: bool,
}

pub fn sync_pack_command(args: SyncPackArgs) -> anyhow::Result<()> {
    let current_dir = current_dir()?;

    let repo_user_settings = RepoUserSettings::read_from_known(&current_dir)?
        .expect("No remote config found in current directory");

    let repo_config = sync_config(&current_dir, &repo_user_settings)?;

    if !repo_config.packs.contains(&args.name) {
        return Err(anyhow::anyhow!(
            "Pack '{}' is not part of the repository",
            args.name
        ));
    }

    let local_pack_config = PackConfig::read_from_named(&current_dir, &args.name)?.ok_or(
        anyhow::anyhow!("Pack config for '{}' not found locally", args.name),
    )?;

    let progress_reporter = if args.silent {
        IndicatifProgressReporter::disabled()
    } else {
        IndicatifProgressReporter::default()
    };

    let index_generator =
        IndexGenerator::from_config(&local_pack_config, &current_dir, progress_reporter.clone())?;

    if args.force_refresh {
        index_generator.clear_cache()?;
    }

    let actual_index = index_generator.index_addons()?;

    let remote_pack_config =
        PackConfig::download_named(repo_user_settings.get_remote(), &args.name)?;

    let remote_index = remote_pack_config.download_indexes(repo_user_settings.get_remote())?;

    let diff = diff_packs(actual_index, remote_index.clone())?;

    if !diff.has_changes() {
        println!("Pack is already up to date.");
        return Ok(());
    }

    println!("{}", diff.to_pretty_string());

    let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to download these changes?")
        .default(false)
        .interact()?;

    if !outcome {
        println!("Aborting sync.");
        return Ok(());
    }

    let diff_applier = local_pack_config.diff_applier(
        &current_dir,
        repo_user_settings.get_remote(),
        progress_reporter,
    );

    diff_applier.apply(diff)?;

    println!("Pack synchronized successfully.");

    Ok(())
}

fn sync_config(
    current_dir: &Path,
    repo_user_settings: &RepoUserSettings,
) -> anyhow::Result<RepoConfig> {
    let remote_url = repo_user_settings.get_remote();

    let remote_repo_config = RepoConfig::download_known(remote_url)?;

    let local_repo_config =
        RepoConfig::read_from_known(current_dir)?.ok_or(anyhow!("Local repo config not found"))?;

    let removed = local_repo_config
        .packs
        .iter()
        .filter(|p| !remote_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    if !removed.is_empty() {
        for pack in removed {
            println!("Pack '{}' has been removed from remote repository.", pack);
            let outcome = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "Do you want to remove the local pack '{}' as well?",
                    pack
                ))
                .default(false)
                .interact()?;
            if outcome {
                delete_pack(current_dir, pack)?;
                println!("Pack '{}' removed locally.", pack);
            }
        }
    }

    let added = remote_repo_config
        .packs
        .iter()
        .filter(|p| !local_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    if !added.is_empty() {
        for pack in added {
            let pack_config = PackConfig::download_named(remote_url, pack)
                .context(format!("Failed to download pack {} configuration", &pack))?;

            pack_config.init_blank_on_fs(current_dir)?;

            println!("Pack '{}' has been added to repository.", pack);
        }
    }

    let existing = remote_repo_config
        .packs
        .iter()
        .filter(|p| local_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in existing {
        let remote_pack_config = PackConfig::download_named(remote_url, pack)
            .context(format!("Failed to download pack {} configuration", &pack))?;
        remote_pack_config.write_to(current_dir)?;
    }

    remote_repo_config.write_to(current_dir)?;

    Ok(remote_repo_config)
}
