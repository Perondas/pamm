use crate::hr_serializable;
use crate::io::serialization::versioning::versioned_types::versioned_optionals_config::VersionedOptionalsConfig;
use crate::io::serialization::versioning::versioned_types::versioned_user_settings::VersionedPackUserSettings;
use crate::migration::version_tag::VersionTag;
use crate::models::pack::pack_config::PackConfig;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;

hr_serializable!(RepoConfig);
hr_serializable!(RepoUserSettings);
hr_serializable!(PackConfig);
hr_serializable!(VersionedPackUserSettings);
hr_serializable!(VersionTag);
hr_serializable!(VersionedOptionalsConfig);
