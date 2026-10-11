use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_pack::GetPack;
use log::debug;

impl ClientRepoHandle {
    pub async fn get_pack_launch_params(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        let (_, settings) = self.get_pack_with_settings(pack_name).await?;

        debug!(
            "Retrieved launch parameters for pack '{}': {:?}",
            pack_name, settings.launch_params
        );

        Ok(settings.launch_params)
    }

    pub async fn get_pack_server_launch_params(
        &self,
        pack_name: &str,
    ) -> anyhow::Result<Vec<String>> {
        let pack_config = self.get_pack(pack_name).await?;

        debug!(
            "Retrieved server-defined launch parameters for pack '{}': {:?}",
            pack_name, pack_config.client_params
        );

        Ok(pack_config.client_params)
    }

    pub async fn set_pack_launch_params(
        &self,
        pack_name: &str,
        launch_params: Vec<String>,
    ) -> anyhow::Result<()> {
        let (_, mut settings) = self.get_pack_with_settings(pack_name).await?;

        debug!(
            "Setting launch parameters for pack '{}': {:?}",
            pack_name, launch_params
        );

        settings.launch_params = launch_params;

        self.write_keyed(&settings, pack_name).await
    }
}
