use anyhow::{anyhow, Context};
use pamm_lib::io::fs::fs_readable::KnownFSReadable;
use pamm_lib::io::fs::fs_writable::IdentifiableFSWritable;
use pamm_lib::io::fs::fs_writable::KnownFSWritable;
use pamm_lib::io::fs::pack::delete_pack::delete_pack;
use pamm_lib::io::net::downloadable::{KnownDownloadable, NamedDownloadable};
use pamm_lib::pack::pack_config::PackConfig;
use pamm_lib::repo::repo_config::RepoConfig;
use pamm_lib::repo::repo_user_settings::RepoUserSettings;
use std::path::Path;

fn sync_config(repo_path: String) -> anyhow::Result<RepoConfig> {
    let repo_path = Path::new(&repo_path);
    let repo_user_settings = RepoUserSettings::read_from_known(repo_path)?
        .ok_or(anyhow!("No remote config found in current directory"))?;

    let remote_url = repo_user_settings.get_remote();

    let remote_repo_config = RepoConfig::download_known(remote_url)?;

    let local_repo_config =
        RepoConfig::read_from_known(repo_path)?.ok_or(anyhow!("Local repo config not found"))?;

    let removed = local_repo_config
        .packs
        .iter()
        .filter(|p| !remote_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in removed {
        delete_pack(repo_path, pack)?;
        println!("Pack '{}' removed locally.", pack);
    }

    let added = remote_repo_config
        .packs
        .iter()
        .filter(|p| !local_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in added {
        let pack_config = PackConfig::download_named(remote_url, pack)
            .context(format!("Failed to download pack {} configuration", &pack))?;

        pack_config.init_blank_on_fs(repo_path)?;
    }

    let existing = remote_repo_config
        .packs
        .iter()
        .filter(|p| local_repo_config.packs.contains(*p))
        .collect::<Vec<_>>();

    for pack in existing {
        let remote_pack_config = PackConfig::download_named(remote_url, pack)
            .context(format!("Failed to download pack {} configuration", &pack))?;
        remote_pack_config.write_to(repo_path)?;
    }

    remote_repo_config.write_to(repo_path)?;

    Ok(remote_repo_config)
}
