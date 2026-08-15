use pamm_lib::models::repo::repo_user_settings::RepoUserSettings;

pub mod load_settings;
pub mod save_settings;

pub struct FlutterRepoUserSettings {
    pub(crate) remote: String,
}

impl From<RepoUserSettings> for FlutterRepoUserSettings {
    fn from(settings: RepoUserSettings) -> Self {
        FlutterRepoUserSettings {
            remote: settings.remote.to_string(),
        }
    }
}

impl TryFrom<FlutterRepoUserSettings> for RepoUserSettings {
    type Error = url::ParseError;

    fn try_from(settings: FlutterRepoUserSettings) -> Result<Self, Self::Error> {
        let remote = settings.remote.parse()?;
        Ok(RepoUserSettings { remote })
    }
}
