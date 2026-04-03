use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::fs::fs_writable::KnownFSWritable;
use crate::migration::version_tag::VersionTag;
use log::{debug, error, info};
use std::path::Path;

pub fn migrate(path: &Path) -> anyhow::Result<()> {
    anyhow::ensure!(
        path.is_dir(),
        "Failed to migrate {:#?} as it's not a folder",
        path
    );

    debug!("Checking {:#?} for migration", path);

    let mut version_tag = VersionTag::read_from_known(path).unwrap_or_default();

    debug!("Version tag for {:#?} is {:#?}", path, version_tag);

    if version_tag.is_latest() {
        info!(
            "{:#?} is already at the latest version, skipping migration",
            path
        );
        return Ok(());
    }

    loop {
        let migration_fn = version_tag.get_migration_function();

        debug!("Migrating {:#?} from version {:#?}", path, version_tag,);

        let new_version_tag = match migration_fn(path) {
            Ok(new_version_tag) => new_version_tag,
            Err(e) => {
                error!(
                    "Failed to migrate {:#?} from version {:#?}: {:?}",
                    path, version_tag, e
                );
                anyhow::bail!(
                    "Failed to migrate {:#?} from version {:#?}: {:?}",
                    path,
                    version_tag,
                    e
                );
            }
        };

        debug!(
            "Migrated {:#?} from version {:#?} to version {:#?}",
            path, version_tag, new_version_tag
        );

        version_tag = new_version_tag;

        version_tag.write_to(path)?;

        if version_tag.is_latest() {
            info!(
                "{:#?} has been migrated to the latest version, stopping migration",
                path
            );
            break;
        }
    }

    Ok(())
}
