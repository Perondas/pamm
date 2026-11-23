use flutter_rust_bridge::frb;
use pamm_lib::application::repo::client::ClientRepoHandle;
pub use pamm_lib::domain::cdlc::Cdlc;
use pamm_lib::domain::pack::pack_config::PackConfig;

#[err_wrap]
pub async fn add_local_pack(
    repo_path: String,
    config: FlutterLocalPackConfig,
) -> anyhow::Result<()> {
    let repot_path = std::path::Path::new(&repo_path);

    let mut handle = ClientRepoHandle::open(repot_path).await?;

    handle.add_local_pack(&config.into()).await
}

#[derive(Debug)]
pub struct FlutterLocalPackConfig {
    pub name: String,
    pub parent: Option<String>,
    pub enabled_cdlc: Option<Cdlc>,
}

impl From<FlutterLocalPackConfig> for PackConfig {
    fn from(value: FlutterLocalPackConfig) -> Self {
        Self::new(
            value.name,
            String::default(),
            Vec::default(),
            value.parent,
            value.enabled_cdlc,
        )
    }
}

#[frb(mirror(Cdlc))]
pub enum _Cdlc {
    GlobalMobilization,
    SogPrairieFire,
    CslaIronCurtain,
    WesternSahara,
    Spearhead1944,
    ReactionForces,
    ExpeditionaryForces,
}
