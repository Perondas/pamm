use crate::io::fs::fs_readable::KnownFSReadable;
use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::serialization::readable::Readable;
use crate::io::serialization::writable::Writable;
use anyhow::Context;
use std::path::Path;

/// Layout version of the on-disk repo format, bumped whenever the file layout
/// changes. Stored in `version.pamm` at the repo root (source, client, and
/// `www/`); a repo without the file is version 1 (the flat pre-versioning
/// layout). Migrations run when the stored version is older than
/// [`CURRENT_REPO_VERSION`].
pub const CURRENT_REPO_VERSION: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepoVersion(pub u32);

impl RepoVersion {
    pub fn current() -> Self {
        Self(CURRENT_REPO_VERSION)
    }

    /// Reads the repo version from `<repo_path>/version.pamm`; a missing file
    /// means version 1 (repos predate the version file).
    pub(crate) fn read_or_v1(repo_path: &Path) -> anyhow::Result<Self> {
        if !repo_path.join(Self::file_name()).exists() {
            return Ok(Self(1));
        }
        Self::read_from_known(repo_path)
    }
}

// Stored as a plain decimal string so the file stays trivially inspectable.
impl Readable for RepoVersion {
    fn from_reader<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let version = content
            .trim()
            .parse::<u32>()
            .with_context(|| format!("Invalid repo version {:?}", content.trim()))?;
        Ok(Self(version))
    }
}

impl Writable for RepoVersion {
    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        write!(writer, "{}", self.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::fs::fs_writable::FixedFsWritable;
    use crate::util::test_utils::TestTempDir;

    #[test]
    fn round_trips_through_the_version_file() {
        let tmp = TestTempDir::new("pamm_repo_version_roundtrip");

        RepoVersion::current().write_to(tmp.path()).unwrap();

        let content = std::fs::read_to_string(tmp.path().join("version.pamm")).unwrap();
        assert_eq!(content, CURRENT_REPO_VERSION.to_string());

        assert_eq!(
            RepoVersion::read_or_v1(tmp.path()).unwrap(),
            RepoVersion::current()
        );
    }

    #[test]
    fn missing_file_means_v1() {
        let tmp = TestTempDir::new("pamm_repo_version_missing");
        assert_eq!(RepoVersion::read_or_v1(tmp.path()).unwrap(), RepoVersion(1));
    }

    #[test]
    fn garbage_content_is_an_error() {
        let tmp = TestTempDir::new("pamm_repo_version_garbage");
        std::fs::write(tmp.path().join("version.pamm"), "not-a-number").unwrap();
        assert!(RepoVersion::read_or_v1(tmp.path()).is_err());
    }
}
