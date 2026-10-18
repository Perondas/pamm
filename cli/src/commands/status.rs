use std::collections::HashMap;
use futures::StreamExt;
use futures::stream;
use pamm_cli_args::commands::status::StatusArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::reading::get_pack::GetPack;
use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;
use termtree::Tree;

pub async fn status_command(_args: StatusArgs) -> anyhow::Result<()> {
    let handle = ClientRepoHandle::open(&std::env::current_dir()?).await?;

    let config = handle.get_config();

    println!("Status for '{}':", config.name);

    if !config.description.is_empty() {
        println!("Description: {}", config.description);
    }

    if config.packs.is_empty() {
        println!("No packs found in the repository.");
        return Ok(());
    }

    let mut root = Tree::new(config.name.clone());

    let mut packs = HashMap::new();

    for pack in config.packs {
        let pack = handle.get_pack(&pack).await?;
        packs.insert(pack.parent, pack.name.clone());
    }

    let current_parent = None;

    loop {

    }


    let mut waiting: Vec<(Option<String>, Tree<String>)> = config.packs.iter().map(async |name| handle.get_pack(name).await).map(|p|).collect();

    loop {}

    todo!()
}
