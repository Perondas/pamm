use crate::hr_serializable;
use crate::pack::config::pack_config::PackConfig;
use crate::repo::remote_config::RemoteConfig;
use crate::repo::repo_config::RepoConfig;
use anyhow::Context;

hr_serializable!(PackConfig);
hr_serializable!(RemoteConfig);
hr_serializable!(RepoConfig);
