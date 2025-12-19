use crate::hr_serializable;
use crate::pack::pack_config::PackConfig;
use crate::repo::repo_config::RepoConfig;
use crate::repo::repo_user_settings::RepoUserSettings;
use anyhow::Context;

hr_serializable!(PackConfig);
hr_serializable!(RepoUserSettings);
hr_serializable!(RepoConfig);
