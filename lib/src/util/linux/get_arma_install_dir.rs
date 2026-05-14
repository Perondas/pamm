use anyhow::{anyhow, Context};
use std::env::home_dir;
use std::fs::read_to_string;
use std::path::PathBuf;
use steam_vdf_parser::{parse_text, Value};

static LIBRARYFOLDERS_PATH: &str = ".steam/root/steamapps/libraryfolders.vdf";
static APPMANIFEST_PATH: &str = ".steam/root/steamapps/appmanifest_107410.acf";

pub fn get_arma_install_dir() -> anyhow::Result<PathBuf> {
    let home_dir = home_dir().ok_or_else(|| anyhow!("Unable to find home directory"))?;

    let libfolders_file = read_to_string(home_dir.join(LIBRARYFOLDERS_PATH))
        .context(anyhow!("Unable to read libraryfolders from path"))?;
    let appmanifest_file = read_to_string(home_dir.join(APPMANIFEST_PATH))
        .context(anyhow!("Unable to read appmanifest from path"))?;

    let libfolders =
        parse_text(&libfolders_file).context(anyhow!("Failed to parse libraryfolders"))?;
    let appmanifest =
        parse_text(&appmanifest_file).context(anyhow!("Failed to parse appmanifest"))?;

    let install_location = libfolders
        .as_obj()
        .context(anyhow!("libraryfolders is not an object"))?
        .iter()
        .map(|(_, value)| arma_in_location(value).map(|contains_arma| (contains_arma, value)))
        .filter_map(|result| match result {
            Ok((true, value)) => Some(value),
            _ => None,
        })
        // I don't think that steam allows more than one install location per app, so we should be good
        .next()
        .context(anyhow!("Arma 3 not found in any library folder"))?;

    let library_path = install_location
        .get("path")
        .context(anyhow!("libraryfolders entry does not contain 'path'"))?
        .as_str()
        .context(anyhow!("libraryfolders path is not a string"))?;

    println!("Library path: {:?}", library_path);

    let arma_dir_name = appmanifest
        .as_obj()
        .context(anyhow!("appmanifest is not an object"))?
        .get("installdir")
        .context(anyhow!("appmanifest does not contain 'installdir'"))?
        .as_str()
        .context(anyhow!("installdir is not a string"))?;

    println!("Arma dir name: {:?}", arma_dir_name);

    Ok(PathBuf::from(format!(
        "{}/steamapps/common/{}",
        library_path, arma_dir_name
    )))
}

fn arma_in_location(value: &Value) -> anyhow::Result<bool> {
    let apps = value
        .get("apps")
        .context(anyhow!("libraryfolders entry does not contain 'apps'"))?;
    apps.get("107410")
        .map(|_| true)
        .ok_or_else(|| anyhow!("Arma 3 not found in this library folder"))
}
