use crate::handle::client::reading::get_pack::GetPack;
use crate::handle::client::client_repo_handle::ClientRepoHandle;
use crate::handle::client::writing::update_pack::UpdatePack;
use crate::models::pack::pack_diff::PackDiff;

pub trait ApplyDiff {
    fn apply_diff(&self, diff: &PackDiff) -> anyhow::Result<()>;
}

impl ApplyDiff for ClientRepoHandle {
    fn apply_diff(&self, diff: &PackDiff) -> anyhow::Result<()> {
        let pack_config = self.get_pack(diff.get_pack_name())?;

        diff.write_index_to_fs(&self.repo_path)?;

        let config = pack_config.update_addons(diff);

        self.update_pack(&config)
    }
}
