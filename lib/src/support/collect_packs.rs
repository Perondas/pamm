use crate::application::pack::read::get_pack::GetPack;
use crate::domain::pack::pack_config::PackConfig;
use crate::domain::pack::pack_user_settings::PackUserSettings;

/// Collects all packs in the chain of the given pack, including the pack itself and all its parents.
pub async fn collect_pack_names(
    handle: &impl GetPack,
    pack_name: &str,
) -> anyhow::Result<Vec<String>> {
    walk_packs(handle, pack_name, |(pack, _)| pack.name.clone()).await
}

pub async fn walk_packs<T>(
    handle: &impl GetPack,
    pack_name: &str,
    action: impl Fn((&PackConfig, &PackUserSettings)) -> T + Send + Sync,
) -> anyhow::Result<Vec<T>> {
    let mut to_visit = Vec::with_capacity(1);
    let mut visited = Vec::with_capacity(1);
    let mut res = Vec::new();

    to_visit.push(pack_name.to_string());

    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        visited.push(current.clone());

        let (pack, user_settings) = handle.get_pack_with_settings(&current).await?;

        res.push(action((&pack, &user_settings)));

        let parents = pack.parents.clone();

        for parent in parents {
            if !visited.contains(&parent) {
                to_visit.push(parent);
            }
        }
    }

    Ok(res)
}
