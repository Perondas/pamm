use pamm_lib::handle::externals::external_addon::ExternalAddon;
use std::path::Path;

pub mod add_external;
pub mod remove_external;
pub mod toggle_externals;

fn externals_to_name(externals: &[ExternalAddon]) -> Vec<String> {
    externals
        .iter()
        .map(|e| {
            e.name.clone().unwrap_or(
                Path::new(&e.path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            )
        })
        .collect()
}
