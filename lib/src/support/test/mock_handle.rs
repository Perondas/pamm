use crate::application::pack::read::get_pack::GetPack;
use crate::application::pack::read::get_repo_info::GetRepoInfo;
use crate::application::pack::write::save_pack_settings::SavePackSettings;
use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_user_settings::PackUserSettings;
use crate::domain::repo::repo_config::RepoConfig;
use std::path::Path;

mockall::mock! {
    pub Handle {}
    impl GetPack for Handle {
        async fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig>;
        async fn get_pack_with_settings(
            &self,
            pack_name: &str,
        ) -> anyhow::Result<(PackConfig, PackUserSettings)>;
        fn get_all_pack_names<'a>(&'a self) -> anyhow::Result<Vec<&'a str>>;
    }
    impl SavePackSettings for Handle {
        fn save_pack_settings<'a,'b>(&self, pack_name: &'a str, settings: &'a PackUserSettings) -> impl Future<Output = anyhow::Result<()>> + Send + 'b;
    }
    impl GetRepoInfo for Handle {
        fn get_repo_path(&self) -> &Path;
        fn get_config(&self) -> &RepoConfig;
    }
}

pub trait MockHandleExt {
    fn mock_pack(
        &mut self,
        pack_name: &'static str,
        parent: Option<&str>,
        optionals: &[&str],
        required: &[&str],
        enabled_optionals: &[&str],
    );
}

impl MockHandleExt for MockHandle {
    fn mock_pack(
        &mut self,
        pack_name: &'static str,
        parent: Option<&str>,
        optionals: &[&str],
        required: &[&str],
        enabled_optionals: &[&str],
    ) {
        let mut config = PackConfig::new(
            pack_name.to_string(),
            "desc".to_string(),
            vec![],
            parent.map(|s| s.to_string()),
            None,
        );

        for opt in optionals {
            config.addons.insert(
                opt.to_string(),
                crate::domain::addon::AddonSettings {
                    is_optional: true,
                    ..Default::default()
                },
            );
        }
        for req in required {
            config.addons.insert(
                req.to_string(),
                crate::domain::addon::AddonSettings {
                    is_optional: false,
                    ..Default::default()
                },
            );
        }

        let mut settings = PackUserSettings::default();
        for opt in enabled_optionals {
            settings.enabled_optionals.insert(opt.to_string());
        }

        let pack_name_owned = pack_name.to_string();
        let config_only = config.clone();
        self.expect_get_pack()
            .withf(move |p| p == pack_name_owned.clone())
            .returning(move |_| Ok(config_only.clone()));

        let pack_name_owned = pack_name.to_string();
        self.expect_get_pack_with_settings()
            .withf(move |p| p == pack_name_owned.clone())
            .returning(move |_| Ok((config.clone(), settings.clone())));

        self.expect_get_all_pack_names()
            .returning(move || Ok(vec![pack_name]));
    }
}
