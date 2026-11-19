use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_user_settings::PackUserSettings;
use crate::domain::repo::repo_config::RepoConfig;
use crate::domain::repo::repo_user_settings::RepoUserSettings;
use crate::domain::repo::server_config::ServerConfig;
use crate::hr_serializable;

hr_serializable!(RepoConfig);
hr_serializable!(RepoUserSettings);
hr_serializable!(PackConfig);
hr_serializable!(PackUserSettings);
hr_serializable!(ServerConfig);
