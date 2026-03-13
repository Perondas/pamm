use crate::hr_serializable;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::settings::pack_user_settings::PackUserSettings;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;

hr_serializable!(RepoConfig);
hr_serializable!(RepoUserSettings);
hr_serializable!(PackConfig);
hr_serializable!(PackUserSettings);
