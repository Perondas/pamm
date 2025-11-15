use crate::consts::{MANIFEST_FILE_NAME, STATE_DIR_NAME};
use crate::pack::pack_diff::PackDiff;
use crate::pack::pack_manifest::PackManifest;
use crate::pack::pack_part::part::PackPart;
use anyhow::Result;
use sha1::{Digest, Sha1};
use std::fs;

impl PackManifest {
    pub fn apply_pack_diff(self, pack_diff: PackDiff) -> Result<Self> {
        let PackManifest { addons, .. } = self;
        let PackDiff {
            added,
            removed,
            modified: changed,
        } = pack_diff;

        let without_removed = addons
            .into_iter()
            .filter(|(path, _)| !removed.contains(path));

        let with_added: Vec<_> = added
            .iter()
            .map(|(path, part)| (path.to_owned(), part.get_checksum()))
            .collect();

        let base_path = std::env::current_dir()?.join(STATE_DIR_NAME);

        for pack in &removed {
            println!("Removed: {}", pack);
            fs::remove_file(base_path.join(pack))?;
        }

        for (path, part) in added {
            println!("Added: {}", path);
            let file_name = format!("{}.part", part.get_rel_path());
            let file = fs::File::create(base_path.join(&file_name))?;
            serde_cbor::to_writer(file, &part)?;
        }

        for (path, part) in changed {
            println!("Updated: {}", path);
            let file_name = format!("{}.part", part.get_rel_path());
            let file = fs::File::create(base_path.join(&file_name))?;
            serde_cbor::to_writer(file, &part)?;
        }

        let manifest = PackManifest {
            addons: without_removed.chain(with_added).collect(),
            pack_checksum: gen_pack_checksum()?,
        };

        let manifest_file = fs::File::create(std::env::current_dir()?.join(MANIFEST_FILE_NAME))?;
        serde_cbor::to_writer(manifest_file, &manifest)?;

        Ok(manifest)
    }
}

fn gen_pack_checksum() -> Result<Vec<u8>> {
    let base_path = std::env::current_dir()?.join(STATE_DIR_NAME);
    let mut files: Vec<_> = fs::read_dir(&base_path)?
        .filter_map(Result::ok)
        .filter(|f| {
            f.file_type().unwrap().is_file()
                && f.path().extension() == Some(std::ffi::OsStr::new("part"))
        })
        .collect();

    files.sort_by_key(|e1| e1.file_name());

    let mut hasher = Sha1::new();
    for entry in files {
        let path = entry.path();
        let part: PackPart = serde_cbor::from_reader(fs::File::open(&path)?)?;
        hasher.update(part.get_checksum());
    }

    let checksum = hasher.finalize().to_vec();
    Ok(checksum)
}
