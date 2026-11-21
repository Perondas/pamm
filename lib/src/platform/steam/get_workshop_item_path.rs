use crate::platform::steam::find_workshop_folder::find_workshop_folder;
use log::debug;
use std::path::PathBuf;

pub fn get_workshop_item_path(id: &str) -> anyhow::Result<Option<PathBuf>> {
    debug!("Looking for workshop item with ID {}", id);

    let workshop_folder = find_workshop_folder()?;

    let item_path = workshop_folder.join(id);

    if item_path.exists() {
        debug!("Found workshop item with ID {} at {:?}", id, item_path);
        return Ok(Some(item_path));
    }

    debug!("Workshop item with ID {} not found at {:?}", id, item_path);
    Ok(None)
}
