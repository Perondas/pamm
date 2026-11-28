use crate::domain::pack::pack_config::PackConfig;
use crate::domain::repo::repo_config::RepoConfig;
use anyhow::{Context, ensure};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::path::{Component, Path, PathBuf};

#[cfg(feature = "client")]
use crate::domain::repo::repo_user_settings::RepoUserSettings;
#[cfg(feature = "client")]
use crate::domain::repo::repo_version::RepoVersion;
#[cfg(feature = "client")]
use crate::infrastructure::fs::files::fs_writable::FixedFsWritable;

#[cfg(feature = "server")]
pub struct ServerRepo;
#[cfg(feature = "client")]
pub struct ClientRepo;

#[cfg(feature = "server")]
pub type TestServerRepoBuilder = TestRepoBuilder<ServerRepo>;
#[cfg(feature = "client")]
pub type TestClientRepoBuilder = TestRepoBuilder<ClientRepo>;

fn default_pack() -> PackConfig {
    PackConfig::new(
        "core".to_string(),
        "test pack".to_string(),
        vec![],
        vec![],
        None,
    )
}

#[cfg(feature = "server")]
pub fn default_server_repo(parent_dir: impl AsRef<Path>) -> TestServerRepoBuilder {
    TestServerRepoBuilder::new(parent_dir)
        .repo_name("repo")
        .description("test repo")
        .pack(default_pack())
}

#[cfg(feature = "client")]
pub fn default_client_repo(parent_dir: impl AsRef<Path>) -> TestClientRepoBuilder {
    TestClientRepoBuilder::new(parent_dir)
        .repo_name("repo")
        .description("test repo")
        .pack(default_pack())
}

pub struct TestRepoBuilder<Kind> {
    parent_dir: PathBuf,
    repo_name: String,
    description: String,
    packs: Vec<PackConfig>,
    #[cfg(feature = "client")]
    local_packs: Vec<PackConfig>,
    files: Vec<(PathBuf, Vec<u8>)>,
    _kind: PhantomData<Kind>,
}

impl<K> TestRepoBuilder<K> {
    pub fn new(parent_dir: impl AsRef<Path>) -> Self {
        Self {
            parent_dir: parent_dir.as_ref().to_path_buf(),
            repo_name: "repo".to_string(),
            description: "test repo".to_string(),
            packs: Vec::new(),
            #[cfg(feature = "client")]
            local_packs: Vec::new(),
            files: Vec::new(),
            _kind: PhantomData,
        }
    }

    pub fn repo_name(mut self, name: impl Into<String>) -> Self {
        self.repo_name = name.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn pack(mut self, pack: PackConfig) -> Self {
        self.packs.push(pack);
        self
    }

    /// Add a file at a path relative to the repo root.
    pub fn file(mut self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Self {
        self.files
            .push((path.as_ref().to_path_buf(), contents.as_ref().to_vec()));
        self
    }

    /// Add a file at a path relative to a pack's root directory.
    pub fn pack_file(
        self,
        pack_name: impl AsRef<Path>,
        path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) -> Self {
        self.file(pack_name.as_ref().join(path), contents)
    }

    fn make_repo_config(&self, packs: HashSet<String>) -> RepoConfig {
        RepoConfig::new(self.repo_name.clone(), self.description.clone(), packs)
    }

    async fn write_files(&self, repo_path: &Path) -> anyhow::Result<()> {
        for (relative_path, contents) in &self.files {
            ensure!(
                relative_path
                    .components()
                    .all(|component| matches!(component, Component::Normal(_))),
                "Test repo file path must be relative and cannot contain '.' or '..': {}",
                relative_path.display()
            );
            ensure!(
                !relative_path.as_os_str().is_empty(),
                "Test repo file path cannot be empty"
            );

            let path = repo_path.join(relative_path);
            let parent = path
                .parent()
                .context("Test repo file path has no parent directory")?;
            tokio::fs::create_dir_all(parent).await?;
            tokio::fs::write(path, contents).await?;
        }

        Ok(())
    }
}

#[cfg(feature = "server")]
impl TestRepoBuilder<ServerRepo> {
    pub async fn build(self) -> anyhow::Result<TestServerRepo> {
        let pack_names = self.packs.iter().map(|pack| pack.name.clone()).collect();
        let config = self.make_repo_config(pack_names);
        let repo_path = config
            .init_server_repo_blank_on_fs(&self.parent_dir)
            .await?;

        for pack in &self.packs {
            pack.init_source_on_fs(&repo_path).await?;
        }

        self.write_files(&repo_path).await?;

        Ok(TestServerRepo { repo_path })
    }
}

#[cfg(feature = "client")]
impl TestRepoBuilder<ClientRepo> {
    pub fn local_pack(mut self, pack: PackConfig) -> Self {
        self.local_packs.push(pack);
        self
    }

    pub async fn build(self) -> anyhow::Result<TestClientRepo> {
        let pack_names = self.packs.iter().map(|pack| pack.name.clone()).collect();
        let config = self.make_repo_config(pack_names);
        let repo_path = self.parent_dir.join(&self.repo_name);
        tokio::fs::create_dir(&repo_path).await?;

        config.write_fixed(&repo_path).await?;
        RepoVersion::current().write_fixed(&repo_path).await?;
        let mut user_settings = RepoUserSettings::new(url::Url::parse("http://localhost/")?);
        user_settings.local_packs = self
            .local_packs
            .iter()
            .map(|pack| pack.name.clone())
            .collect();
        user_settings.write_fixed(&repo_path).await?;

        for pack in &self.packs {
            pack.init_client_on_fs(&repo_path).await?;
        }
        for pack in &self.local_packs {
            pack.init_client_on_fs(&repo_path).await?;
        }

        self.write_files(&repo_path).await?;

        Ok(TestClientRepo { repo_path })
    }
}

#[cfg(any(feature = "client", feature = "server"))]
pub struct TestRepoFixture {
    repo_path: PathBuf,
}

#[cfg(any(feature = "client", feature = "server"))]
impl TestRepoFixture {
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}

#[cfg(feature = "server")]
pub type TestServerRepo = TestRepoFixture;

#[cfg(feature = "client")]
pub type TestClientRepo = TestRepoFixture;

#[cfg(all(test, feature = "server"))]
mod server_tests {
    use super::*;
    use crate::support::test::test_utils::TestTempDir;

    #[tokio::test]
    async fn builds_server_repo_with_pack_and_files() {
        let temp_dir = TestTempDir::new("pamm_test_server_repo_builder");
        let fixture = TestServerRepoBuilder::new(temp_dir.path())
            .repo_name("fixture")
            .description("test server repo")
            .pack(PackConfig::new(
                "core".to_string(),
                "core pack".to_string(),
                vec![],
                vec![],
                None,
            ))
            .pack_file("core", "addons/@addon/file.txt", b"addon data".as_slice())
            .file("media/readme.txt", b"repo data".as_slice())
            .build()
            .await
            .unwrap();

        assert!(fixture.repo_path().join("core/pack.config.json").is_file());
        assert_eq!(
            std::fs::read(fixture.repo_path().join("core/addons/@addon/file.txt")).unwrap(),
            b"addon data"
        );
        assert_eq!(
            std::fs::read(fixture.repo_path().join("media/readme.txt")).unwrap(),
            b"repo data"
        );
    }
}

#[cfg(all(test, feature = "client"))]
mod client_tests {
    use super::*;
    use crate::application::pack::read::get_pack::GetPack;
    use crate::application::pack::read::get_repo_info::GetRepoInfo;
    use crate::application::repo::client::ClientRepoHandle;
    use crate::support::test::test_utils::TestTempDir;

    #[tokio::test]
    async fn builds_client_repo_with_pack_and_files() {
        let temp_dir = TestTempDir::new("pamm_test_client_repo_builder");
        let fixture = TestClientRepoBuilder::new(temp_dir.path())
            .repo_name("fixture")
            .description("test client repo")
            .pack(PackConfig::new(
                "core".to_string(),
                "core pack".to_string(),
                vec![],
                vec![],
                None,
            ))
            .pack_file("core", "addons/@addon/file.txt", b"addon data".as_slice())
            .file("media/readme.txt", b"repo data".as_slice())
            .build()
            .await
            .unwrap();

        assert!(fixture.repo_path().join("core/pack.config.json").is_file());
        assert!(
            fixture
                .repo_path()
                .join("core/pack.settings.json")
                .is_file()
        );
        assert_eq!(
            std::fs::read(fixture.repo_path().join("core/addons/@addon/file.txt")).unwrap(),
            b"addon data"
        );
        assert_eq!(
            std::fs::read(fixture.repo_path().join("media/readme.txt")).unwrap(),
            b"repo data"
        );
    }

    #[tokio::test]
    async fn builds_client_repo_with_local_pack() {
        let temp_dir = TestTempDir::new("pamm_test_client_repo_builder_local_pack");
        let fixture = TestClientRepoBuilder::new(temp_dir.path())
            .local_pack(PackConfig::new(
                "mission".to_string(),
                "local mission".to_string(),
                vec![],
                vec![],
                None,
            ))
            .pack_file(
                "mission",
                "addons/@addon/file.txt",
                b"local data".as_slice(),
            )
            .build()
            .await
            .unwrap();
        let repo = ClientRepoHandle::open(fixture.repo_path()).await.unwrap();

        assert!(repo.is_local_pack("mission"));
        assert!(!repo.get_config().packs.contains("mission"));
        assert_eq!(repo.get_pack("mission").await.unwrap().name, "mission");
        assert_eq!(
            std::fs::read(fixture.repo_path().join("mission/addons/@addon/file.txt")).unwrap(),
            b"local data"
        );
    }
}
