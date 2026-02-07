use std::path::Path;
use anyhow::ensure;
use pamm_lib::commands::steam_launch::launch_via_steam;

pub fn launch(repo_dir: String, pack_name: String) -> anyhow::Result<()> {
    let repo_dir = Path::new(&repo_dir);
    
    ensure!(!pack_name.is_empty(), "Pack name must be provided");
    ensure!(repo_dir.is_dir(), "Repo dir is not a directory");
    
    launch_via_steam(repo_dir, &pack_name)
}