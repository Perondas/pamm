use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::models::pack::pack_config::PackConfig;

pub fn add_local_pack(repo_path: String, config: FlutterLocalPackConfig) -> anyhow::Result<()> {
    let repot_path = std::path::Path::new(&repo_path);

    let mut handle = ClientRepoHandle::open(repot_path)?;

    handle.add_local_pack(&config.into())
}

#[derive(Debug)]
pub struct FlutterLocalPackConfig {
    pub name: String,
    pub parent: Option<String>,
}

impl From<FlutterLocalPackConfig> for PackConfig {
    fn from(value: FlutterLocalPackConfig) -> Self {
        Self::new(value.name, String::default(), Vec::default(), value.parent)
    }
}
