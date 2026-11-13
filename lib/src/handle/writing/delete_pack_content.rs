use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::fs::rm::rm_pack_content::remove_pack_content;
use anyhow::{Context, anyhow, ensure};
use log::debug;

pub trait DeletePackContent {
    fn delete_pack_content(
        &mut self,
        pack_name: &str,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl<T: GetPack + GetRepoInfo + Send> DeletePackContent for T {
    async fn delete_pack_content(&mut self, pack_name: &str) -> anyhow::Result<()> {
        ensure!(
            self.get_all_pack_names().await?.contains(&pack_name),
            "Pack '{}' not found in repo",
            pack_name
        );

        debug!("Deleting content of pack '{}'", pack_name);

        remove_pack_content(&self.get_repo_path(), pack_name)
            .await
            .context(anyhow!("Failed to remove pack content"))?;

        debug!("Successfully deleted content of pack '{}'", pack_name);

        Ok(())
    }
}

#[cfg(all(test, feature = "server"))]
mod server_tests {
    use super::*;
    use crate::handle::repo_handle::RepoHandle;
    use crate::handle::server_repo_handle::ServerRepoHandle;
    use crate::io::files::name_consts::{ADDONS_DIR_NAME, INDEX_DIR_NAME};
    use crate::models::pack::pack_config::PackConfig;
    use crate::util::test::repo_builder::TestServerRepoBuilder;
    use crate::util::test::test_utils::TestTempDir;

    #[tokio::test]
    async fn delete_server_pack_content_clears_addons_and_indexes() {
        let temp_dir = TestTempDir::new("pamm_delete_server_pack_content");
        let fixture = TestServerRepoBuilder::new(temp_dir.path())
            .pack(PackConfig::new(
                "core".to_string(),
                "test pack".to_string(),
                vec![],
                None,
                None,
            ))
            .pack_file("core", "addons/@addon/file.txt", b"addon data".as_slice())
            .pack_file("core", "indexes/core.index", b"index data".as_slice())
            .build()
            .await
            .unwrap();
        let mut repo: ServerRepoHandle = RepoHandle::open(fixture.repo_path())
            .await
            .unwrap()
            .try_into()
            .unwrap();

        repo.delete_pack_content("core").await.unwrap();

        let pack_path = fixture.repo_path().join("core");
        for dir in [ADDONS_DIR_NAME, INDEX_DIR_NAME] {
            let path = pack_path.join(dir);
            assert!(path.is_dir(), "{dir} directory should remain");
            assert!(
                std::fs::read_dir(path).unwrap().next().is_none(),
                "{dir} directory should be empty"
            );
        }
    }
}

#[cfg(all(test, feature = "client"))]
mod client_tests {
    use super::*;
    use crate::handle::client_repo_handle::ClientRepoHandle;
    use crate::handle::repo_handle::RepoHandle;
    use crate::io::files::name_consts::{ADDONS_DIR_NAME, INDEX_DIR_NAME};
    use crate::models::pack::pack_config::PackConfig;
    use crate::util::test::repo_builder::TestClientRepoBuilder;
    use crate::util::test::test_utils::TestTempDir;

    #[tokio::test]
    async fn delete_client_pack_content_clears_addons_and_indexes() {
        let temp_dir = TestTempDir::new("pamm_delete_client_pack_content");
        let fixture = TestClientRepoBuilder::new(temp_dir.path())
            .pack(PackConfig::new(
                "core".to_string(),
                "test pack".to_string(),
                vec![],
                None,
                None,
            ))
            .pack_file("core", "addons/@addon/file.txt", b"addon data".as_slice())
            .pack_file("core", "indexes/core.index", b"index data".as_slice())
            .build()
            .await
            .unwrap();
        let mut repo: ClientRepoHandle = RepoHandle::open(fixture.repo_path())
            .await
            .unwrap()
            .try_into()
            .unwrap();

        repo.delete_pack_content("core").await.unwrap();

        let pack_path = fixture.repo_path().join("core");
        for dir in [ADDONS_DIR_NAME, INDEX_DIR_NAME] {
            let path = pack_path.join(dir);
            assert!(path.is_dir(), "{dir} directory should remain");
            assert!(
                std::fs::read_dir(path).unwrap().next().is_none(),
                "{dir} directory should be empty"
            );
        }
    }
}
