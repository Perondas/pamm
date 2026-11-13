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


#[cfg(test)]
mod test {
    use crate::util::test::test_utils::TestTempDir;

    #[test]
    fn test_delete_pack_content() {
        let dir = TestTempDir::new("test_delete_pack_content");



    }
}
