use crate::hr_serializable;
use crate::pack::pack_config::PackConfig;
use crate::pack::pack_user_settings::PackUserSettings;
use crate::repo::repo_config::RepoConfig;
use crate::repo::repo_user_settings::RepoUserSettings;

hr_serializable!(RepoConfig);
hr_serializable!(RepoUserSettings);
hr_serializable!(PackConfig);
hr_serializable!(PackUserSettings);
