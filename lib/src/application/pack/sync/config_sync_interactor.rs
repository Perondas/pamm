pub trait ConfigSyncInteractor {
    fn confirm_pack_removal(&self, pack_name: &str) -> anyhow::Result<bool>;
    fn notify_pack_added(&self, pack_name: &str) -> anyhow::Result<()>;
}

/// A dummy interactor that always confirms pack removal and does nothing on pack addition.
pub struct DummyConfigSyncInteractor;

impl ConfigSyncInteractor for DummyConfigSyncInteractor {
    fn confirm_pack_removal(&self, _pack_name: &str) -> anyhow::Result<bool> {
        Ok(true)
    }

    fn notify_pack_added(&self, _pack_name: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
