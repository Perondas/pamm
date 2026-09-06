use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::io::fs::pack::index_generator::IndexGenerator;
use crate::io::progress_reporting::progress_reporter::ProgressReporter;
use crate::models::pack::pack_config::PackConfig;
use anyhow::{Context, ensure};

impl ClientRepoHandle {
    pub fn sync_local_pack(
        &self,
        pack_name: &str,
        progress_reporter: &impl ProgressReporter,
    ) -> anyhow::Result<()> {
        ensure!(
            self.user_settings()
                .local_packs
                .contains(&pack_name.to_string()),
            "Local Pack '{}' not found",
            pack_name
        );

        // Index from source (uses the sled cache at <pack>/addons/.cache/).
        let index_generator =
            IndexGenerator::from_handle(self, pack_name, progress_reporter.clone())?;

        let pack_index = index_generator
            .index_addons()
            .context("Failed to index pack addons during build")?;

        let mut config: PackConfig = self
            .read_keyed(pack_name)
            .context("Failed to read pack config during build")?;

        // Update the pack config
        for index in &pack_index.addons {
            config.addons.entry(index.name.clone()).or_default();
        }

        self.write(&config)
            .context("Failed to write updated pack config during build")?;

        Ok(())
    }
}
