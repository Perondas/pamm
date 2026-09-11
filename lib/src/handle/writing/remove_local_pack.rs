use crate::handle::client_repo_handle::ClientRepoHandle;
use crate::handle::reading::get_pack::GetPack;
use crate::io::fs::rm::rm_pack::remove_pack;
use anyhow::{Context, anyhow, bail, ensure};

impl ClientRepoHandle {
    pub fn remove_local_pack(&mut self, name: &str) -> anyhow::Result<()> {
        let mut settings = self.user_settings().clone();

        ensure!(
            settings.local_packs.contains(&name.to_string()),
            "Local pack '{}' not found in repo",
            name
        );

        if let Some(dependant) = self
            .has_no_dependants(name)
            .context(anyhow!("Failed to check dependants"))?
        {
            bail!(
                "Cannot remove local pack '{}' because it is a parent of local pack '{}'",
                name,
                dependant
            );
        }

        settings.local_packs.retain(|p| p != name);

        self.update_settings(settings).context(anyhow!(
            "failed to update user settings after removing local pack: '{}'",
            name,
        ))?;

        // Delete on disk
        remove_pack(&self.repo_path, name).context(anyhow!(
            "failed to remove pack: '{}' from disk. Remove manually!",
            name
        ))
    }

    fn has_no_dependants(&self, name: &str) -> anyhow::Result<Option<String>> {
        let settings = self.user_settings().clone();

        let others = settings.local_packs.iter().filter(|p| *p != name);

        for other in others {
            let config = self.get_pack(other).context(anyhow!(
                "failed to read pack config for local pack: '{}'",
                other
            ))?;

            if matches!(config.parent.as_deref(), Some(parent) if parent == name) {
                return Ok(Some(other.clone()));
            }
        }

        Ok(None)
    }
}
