use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::handle::writing::save_pack_settings::SavePackSettings;
use crate::models::pack::pack_config::PackConfig;
use crate::models::pack::pack_user_settings::PackUserSettings;
use crate::models::repo::repo_config::RepoConfig;
use crate::models::repo::repo_user_settings::RepoUserSettings;
use std::path::Path;

mockall::mock! {
    pub Handle {}
    impl GetPack for Handle {
        fn get_pack(&self, pack_name: &str) -> anyhow::Result<PackConfig>;
        fn get_pack_with_settings(
            &self,
            pack_name: &str,
        ) -> anyhow::Result<(PackConfig, PackUserSettings)>;
    }
    impl SavePackSettings for Handle {
        fn save_pack_settings(&self, pack_name: &str, settings: &PackUserSettings) -> anyhow::Result<()>;
    }
    impl GetRepoInfo for Handle {
        fn get_repo_path(&self) -> &Path;
        fn get_config(&self) -> &RepoConfig;
        fn get_repo_user_settings<'a>(&'a self) -> anyhow::Result<&'a RepoUserSettings>;
    }
}

pub trait MockHandleExt {
    fn mock_pack(
        &mut self,
        pack_name: &str,
        parent: Option<&str>,
        optionals: &[&str],
        required: &[&str],
        enabled_optionals: &[&str],
    );
}

impl MockHandleExt for MockHandle {
    fn mock_pack(
        &mut self,
        pack_name: &str,
        parent: Option<&str>,
        optionals: &[&str],
        required: &[&str],
        enabled_optionals: &[&str],
    ) {
        let mut config = PackConfig::new(
            pack_name.to_string(),
            "desc".to_string(),
            vec![],
            vec![],
            parent.map(|s| s.to_string()),
        );

        for opt in optionals {
            config.addons.insert(
                opt.to_string(),
                crate::models::pack::addon::AddonSettings { is_optional: true },
            );
        }
        for req in required {
            config.addons.insert(
                req.to_string(),
                crate::models::pack::addon::AddonSettings { is_optional: false },
            );
        }

        let mut settings = PackUserSettings::default();
        for opt in enabled_optionals {
            settings.enabled_optionals.insert(opt.to_string());
        }

        let pack_name_owned = pack_name.to_string();
        self.expect_get_pack_with_settings()
            .withf(move |p| p == pack_name_owned)
            .returning(move |_| Ok((config.clone(), settings.clone())));
    }
}
