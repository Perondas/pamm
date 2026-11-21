use crate::domain::cdlc::Cdlc;
use crate::platform::steam::arma_install::find_arma_install;
use log::{debug, info};

pub fn has_cdlc_installed(cldc: &Cdlc) -> anyhow::Result<bool> {
    let arma_install = find_arma_install()?;

    let cdlc_code = cldc.short_form();

    let candidates = [
        arma_install.install_dir.join(cdlc_code.to_uppercase()),
        arma_install.install_dir.join(cdlc_code.to_lowercase()),
    ];

    for candidate in candidates {
        let cdlc_folder = arma_install.install_dir.join(candidate);

        debug!("Checking for CDLC folder at {:?}", cdlc_folder);

        if !cdlc_folder.exists() {
            debug!("CDLC folder does not exist: {:?}", cdlc_folder);
            continue;
        }

        if !cdlc_folder.join("mod.cpp").exists() {
            debug!(
                "CDLC folder exists but mod.cpp is missing: {:?}",
                cdlc_folder
            );
            continue;
        }

        info!("CDLC {:?} seems to be installed at {:?}", cldc, cdlc_folder);
        return Ok(true);
    }

    Ok(false)
}
