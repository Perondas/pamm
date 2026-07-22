use crate::io::files::file_names::fixed_file::FixedFile;
use crate::io::net::downloadable::KnownDownloadable;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_version::{CURRENT_REPO_VERSION, RepoVersion};
use anyhow::bail;
use url::Url;

/// Ensure the remote repo uses the layout version this client understands.
///
/// A remote without `version.pamm` is a v1 (flat-layout) repo: its URLs no
/// longer match what this client requests, so we fail with a clear message
/// instead of a confusing 404 later. To tell "missing version file" apart from
/// "remote unreachable", the repo config is probed as a fallback before
/// blaming the layout.
pub(crate) fn verify_remote_version(remote: &Url) -> anyhow::Result<()> {
    match RepoVersion::download_known(remote) {
        Ok(RepoVersion(CURRENT_REPO_VERSION)) => Ok(()),
        Ok(RepoVersion(version)) if version > CURRENT_REPO_VERSION => bail!(
            "Remote repo at {} has layout version {} but this pamm only supports up to {}; \
             update pamm",
            remote,
            version,
            CURRENT_REPO_VERSION
        ),
        Ok(RepoVersion(version)) => bail!(
            "Remote repo at {} still uses layout version {}; rebuild the server repo with a \
             current pamm version",
            remote,
            version
        ),
        Err(version_err) => {
            if RepoConfig::download_known(remote).is_ok() {
                bail!(
                    "Remote repo at {} has no {} — it still uses the old v1 layout; rebuild \
                     the server repo with a current pamm version",
                    remote,
                    RepoVersion::file_name()
                );
            }
            Err(version_err.context(format!("Failed to reach remote repo at {}", remote)))
        }
    }
}
