use anyhow::bail;
use pamm_cli_args::commands::status::StatusArgs;
use pamm_lib::handle::client_repo_handle::ClientRepoHandle;
use pamm_lib::handle::reading::get_pack::GetPack;
use pamm_lib::handle::reading::get_repo_info::GetRepoInfo;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use termtree::Tree;

#[derive(Debug)]
struct Node {
    pack_name: String,
    needs_update: bool,
}

impl Node {
    fn new(pack_name: String, needs_update: bool) -> Self {
        Self {
            pack_name,
            needs_update,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.pack_name,
            if self.needs_update {
                " (needs update)"
            } else {
                ""
            }
        )
    }
}

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

    let mut repo_root = Tree::new(Node::new(config.name.clone(), false));

    let mut packs = HashMap::new();

    for pack in handle.get_all_packs() {
        let pack = handle.get_pack(pack).await?;
        let needs_update = !handle
            .quick_check_pack_up_to_date(&pack.name)
            .await
            .unwrap_or(true);

        let node = Node::new(pack.name.clone(), needs_update);

        packs
            .entry(pack.parent.clone())
            .or_insert_with(Vec::new)
            .push(Tree::new(node));
    }

    let root_packs = packs.remove(&None).unwrap_or_default();

    if root_packs.is_empty() && !packs.is_empty() {
        bail!(
            "No root pack found, but there are child packs. The repository may be in an inconsistent state."
        );
    }

    repo_root.leaves = root_packs;

    let mut current_level = repo_root.leaves.iter_mut().collect::<Vec<_>>();

    while !packs.is_empty() {
        if current_level.is_empty() && !packs.is_empty() {
            bail!(
                "There are child packs with no path to the root. The repository may be in an inconsistent state. These are the offending packs: {:?}",
                packs
                    .values()
                    .flatten()
                    .map(|node| node.root.pack_name.as_ref())
                    .collect::<Vec<&str>>()
            )
        }

        let mut next_level = vec![];
        for node in current_level {
            let children = packs
                .remove(&Some(node.root.pack_name.clone()))
                .unwrap_or_default();
            node.leaves = children;

            next_level.extend(node.leaves.iter_mut());
        }

        current_level = next_level;
    }

    println!("{}", repo_root);

    Ok(())
}
