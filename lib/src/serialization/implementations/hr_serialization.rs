use crate::hr_serializable;
use crate::pack::pack_config::PackConfig;
use crate::repo::local_repo_config::LocalRepoConfig;
use crate::repo::repo_config::RepoConfig;
use anyhow::Context;

hr_serializable!(PackConfig);
hr_serializable!(LocalRepoConfig);
hr_serializable!(RepoConfig);
