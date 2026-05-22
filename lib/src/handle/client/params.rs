use crate::handle::client::reading::get_pack::GetPack;
use crate::handle::client::client_repo_handle::ClientRepoHandle;
use log::debug;

impl ClientRepoHandle {
    pub fn get_pack_launch_params(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        let (_, settings) = self.get_pack_with_settings(pack_name)?;

        debug!(
            "Retrieved launch parameters for pack '{}': {:?}",
            pack_name, settings.launch_params
        );

        Ok(settings.launch_params)
    }

    pub fn get_pack_server_launch_params(&self, pack_name: &str) -> anyhow::Result<Vec<String>> {
        let pack_config = self.get_pack(pack_name)?;

        debug!(
            "Retrieved server-defined launch parameters for pack '{}': {:?}",
            pack_name, pack_config.client_params
        );

        Ok(pack_config.client_params)
    }

    pub fn set_pack_launch_params(
        &self,
        pack_name: &str,
        launch_params: Vec<String>,
    ) -> anyhow::Result<()> {
        let (_, mut settings) = self.get_pack_with_settings(pack_name)?;

        debug!(
            "Setting launch parameters for pack '{}': {:?}",
            pack_name, launch_params
        );

        settings.launch_params = launch_params;

        self.write_named(&settings, pack_name)
    }
}
