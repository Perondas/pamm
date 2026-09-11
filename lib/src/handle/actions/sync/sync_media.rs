use crate::io::files::file_paths::rel_path::RelPath;
use crate::io::files::name_consts::MEDIA_DIR_NAME;
use crate::io::net::download_file::download_file_unverified;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use log::warn;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use url::Url;

/// Download every media file referenced by the repo and pack configs into
/// `<repo>/media/`, and prune local media files that are no longer referenced
/// (including `.part` leftovers of interrupted downloads).
///
/// Referenced files are always re-downloaded: they are small, this only runs
/// on user-initiated config syncs, and nothing indexes media for change
/// detection.
///
/// Media is best-effort cosmetics: every failure is logged and skipped, this
/// never fails the surrounding init/sync operation.
pub(crate) fn download_referenced_media(
    repo_path: &Path,
    remote: &Url,
    repo_config: &RepoConfig,
    pack_configs: &[PackConfig],
) {
    let repo_names = repo_config
        .customization
        .iter()
        .flat_map(|c| c.media_file_names());
    let pack_names = pack_configs
        .iter()
        .flat_map(|p| p.customization.iter())
        .flat_map(|c| c.media_file_names());

    let referenced = repo_names
        .chain(pack_names)
        .filter(|name| {
            // A media reference is a plain file name inside media/; anything
            // path-like is rejected so a remote config can't escape the dir.
            let valid = !name.contains('/') && !name.contains('\\') && *name != "..";
            if !valid {
                warn!("Ignoring invalid media file name '{}'", name);
            }
            valid
        })
        .collect::<HashSet<_>>();

    let media_dir = RelPath::from_name(MEDIA_DIR_NAME).with_base_path(repo_path);

    if referenced.is_empty() && !media_dir.is_dir() {
        return;
    }

    if let Err(e) = fs::create_dir_all(&media_dir) {
        warn!("Failed to create media directory {:?}: {}", media_dir, e);
        return;
    }

    for name in &referenced {
        let rel = RelPath::from_name(MEDIA_DIR_NAME).push(name);
        if let Err(e) =
            download_file_unverified(&rel.with_base_path(repo_path), rel.with_base_url(remote))
        {
            warn!("Failed to download media file '{}': {}", name, e);
        }
    }

    prune_unreferenced(&media_dir, &referenced);
}

fn prune_unreferenced(media_dir: &Path, referenced: &HashSet<&str>) {
    let entries = match fs::read_dir(media_dir) {
        Ok(entries) => entries,
        Err(e) => {
            warn!("Failed to read media directory {:?}: {}", media_dir, e);
            return;
        }
    };

    for entry in entries.flatten() {
        if !entry.path().is_file() {
            continue;
        }
        let name = entry.file_name();
        if referenced.contains(name.to_string_lossy().as_ref()) {
            continue;
        }
        if let Err(e) = fs::remove_file(entry.path()) {
            warn!(
                "Failed to remove stale media file {:?}: {}",
                entry.path(),
                e
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::repo::repo_customization::RepoCustomization;
    use crate::util::test_utils::TestTempDir;

    // Unreferenced files (incl. .part leftovers) are pruned, referenced files
    // survive even when their re-download fails, and path-like names coming
    // from a (potentially malicious) remote config are ignored.
    #[test]
    fn prunes_unreferenced_rejects_invalid_and_tolerates_failures() {
        let tmp = TestTempDir::new("pamm_sync_media_prune");
        let repo_path = tmp.path();
        let media_dir = repo_path.join(MEDIA_DIR_NAME);
        fs::create_dir_all(&media_dir).unwrap();
        fs::write(media_dir.join("keep.png"), b"keep").unwrap();
        fs::write(media_dir.join("old.png"), b"old").unwrap();
        fs::write(media_dir.join("interrupted.png.part"), b"part").unwrap();

        let mut repo_config =
            RepoConfig::new("repo".to_string(), "test".to_string(), Default::default());
        repo_config.customization = Some(RepoCustomization {
            color: None,
            icon: Some("keep.png".to_string()),
            banner: Some("../escape.png".to_string()),
        });

        // Nothing listens here: every download fails and is tolerated.
        let remote = Url::parse("http://127.0.0.1:1/").unwrap();
        download_referenced_media(repo_path, &remote, &repo_config, &[]);

        assert_eq!(fs::read(media_dir.join("keep.png")).unwrap(), b"keep");
        assert!(!media_dir.join("old.png").exists());
        assert!(!media_dir.join("interrupted.png.part").exists());
        assert!(!repo_path.join("escape.png").exists());
    }
}
