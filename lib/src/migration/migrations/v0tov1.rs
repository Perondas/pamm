use crate::migration::version_tag::VersionTag;
use std::path::Path;

pub fn v0_to_v1(_path: &Path) -> anyhow::Result<VersionTag> {
    Ok(VersionTag::V1)
}
