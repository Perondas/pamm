use comfy_table::{ContentArrangement, Table, presets};
use pamm_cli_args::commands::status::StatusArgs;
use pamm_lib::application::pack::read::get_pack::GetPack;
use pamm_lib::application::pack::read::get_repo_info::GetRepoInfo;
use pamm_lib::application::repo::client::ClientRepoHandle;
use pamm_lib::application::repo::handle::RepoHandle;
use pamm_lib::domain::cdlc::Cdlc;
use std::env::current_dir;

/// One line of the overview table. Columns only a client repo can fill are
/// `None` for server repos and are left out of the table entirely.
struct PackRow {
    name: String,
    kind: Option<&'static str>,
    status: Option<&'static str>,
    parents: String,
}

pub async fn status_command(args: StatusArgs) -> anyhow::Result<()> {
    let handle = RepoHandle::open(&current_dir()?).await?;

    if let Some(name) = args.name {
        return pack_status(&handle, &name).await;
    }

    let config = handle.get_config();

    println!(
        "Status for '{}' ({} repository):",
        config.name,
        repo_kind(&handle)
    );

    if !config.description.is_empty() {
        println!("Description: {}", config.description);
    }

    let mut names = handle.get_all_pack_names()?;

    if names.is_empty() {
        println!("No packs found in the repository.");
        return Ok(());
    }

    names.sort_unstable();

    let mut rows = Vec::with_capacity(names.len());
    let mut failures = Vec::new();

    for name in names {
        let pack = handle.get_pack(name).await?;

        let (kind, status) = match &handle {
            RepoHandle::Client(client) => {
                let status = if client.is_local_pack(name) {
                    "-"
                } else {
                    match update_status(client, name).await {
                        Ok(status) => status,
                        Err(e) => {
                            failures.push((name, e));
                            "Unknown"
                        }
                    }
                };

                (Some(pack_kind(client, name)), Some(status))
            }
            RepoHandle::Server(_) => (None, None),
        };

        rows.push(PackRow {
            name: pack.name,
            kind,
            status,
            parents: format_parents(&pack.parents, "-"),
        });
    }

    println!();
    print_table(&rows);

    if !failures.is_empty() {
        println!();
        for (name, e) in failures {
            println!("Unable to check '{name}': {e:#}");
        }
    }

    Ok(())
}

async fn pack_status(handle: &RepoHandle, pack_name: &str) -> anyhow::Result<()> {
    let pack = handle.get_pack(pack_name).await?;

    println!("Pack: {}", pack.name);

    if !pack.description.is_empty() {
        println!("Description: {}", pack.description);
    }

    println!("Parents: {}", format_parents(&pack.parents, "None"));
    println!(
        "CDLC: {}",
        pack.enabled_cdlc.as_ref().map_or("None", Cdlc::name)
    );

    if let RepoHandle::Client(client) = handle {
        println!("Kind: {}", pack_kind(client, pack_name));

        let status = if client.is_local_pack(pack_name) {
            "Local pack (no remote)".to_string()
        } else {
            match update_status(client, pack_name).await {
                Ok(status) => status.to_string(),
                Err(e) => format!("Unable to check ({e:#})"),
            }
        };

        println!("Status: {}", status);
    }

    Ok(())
}

fn repo_kind(handle: &RepoHandle) -> &'static str {
    match handle {
        RepoHandle::Client(_) => "client",
        RepoHandle::Server(_) => "server",
    }
}

fn pack_kind(client: &ClientRepoHandle, pack_name: &str) -> &'static str {
    if client.is_local_pack(pack_name) {
        "local"
    } else {
        "remote"
    }
}

/// Quick checks a remote pack against the server. Must not be called for local packs.
async fn update_status(client: &ClientRepoHandle, pack_name: &str) -> anyhow::Result<&'static str> {
    Ok(if client.quick_check_pack_up_to_date(pack_name).await? {
        "Up to date"
    } else {
        "Needs update"
    })
}

fn format_parents(parents: &[String], none: &str) -> String {
    if parents.is_empty() {
        none.to_string()
    } else {
        parents.join(", ")
    }
}

/// Prints the rows as a table, skipping any column no row has a value for.
fn print_table(rows: &[PackRow]) {
    let mut columns: Vec<(&str, Vec<&str>)> =
        vec![("Pack", rows.iter().map(|r| r.name.as_str()).collect())];

    if rows.iter().any(|r| r.kind.is_some()) {
        columns.push(("Kind", rows.iter().map(|r| r.kind.unwrap_or("-")).collect()));
    }

    if rows.iter().any(|r| r.status.is_some()) {
        columns.push((
            "Status",
            rows.iter().map(|r| r.status.unwrap_or("-")).collect(),
        ));
    }

    columns.push(("Parents", rows.iter().map(|r| r.parents.as_str()).collect()));

    let mut table = Table::new();
    table
        .load_style(presets::ASCII_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(columns.iter().map(|(header, _)| *header));

    for i in 0..rows.len() {
        table.add_row(columns.iter().map(|(_, cells)| cells[i]));
    }

    println!("{table}");
}
