use crate::application::pack::read::get_pack::GetPack;

/// Collects all packs in the chain of the given pack, including the pack itself and all its parents.
#[cfg(any(feature = "client", feature = "deploy"))]
pub async fn collect_pack_names(
    handle: &impl GetPack,
    pack_name: &str,
) -> anyhow::Result<Vec<String>> {
    walk_packs(handle, pack_name, |_, name| async move { Ok(name) }).await
}

/// Visits the given pack and all its ancestors, awaiting `action` with the
/// handle and the name of each.
///
/// Only the pack configs are read to find the parents; `action` reads whatever
/// else it needs (e.g. the user settings) itself.
pub async fn walk_packs<'h, H, T, F, Fut>(
    handle: &'h H,
    pack_name: &str,
    action: F,
) -> anyhow::Result<Vec<T>>
where
    H: GetPack,
    F: Fn(&'h H, String) -> Fut,
    Fut: Future<Output = anyhow::Result<T>>,
{
    let mut to_visit = Vec::with_capacity(1);
    let mut visited = Vec::with_capacity(1);
    let mut res = Vec::new();

    to_visit.push(pack_name.to_string());

    while let Some(current) = to_visit.pop() {
        visited.push(current.clone());

        let pack = handle.get_pack(&current).await?;

        res.push(action(handle, current).await?);

        for parent in pack.parents {
            if !visited.contains(&parent) && !to_visit.contains(&parent) {
                to_visit.push(parent);
            }
        }
    }

    Ok(res)
}
