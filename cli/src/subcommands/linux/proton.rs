use anyhow::Context;
use pamm_lib::platform::proton::proton_env::Proton;
use pamm_lib::platform::steam::arma_install::find_arma_install;

/// Arma's Proton setup, detected fresh.
pub(super) fn detect_proton() -> anyhow::Result<Proton> {
    let arma = find_arma_install().context("Failed to find the Arma 3 installation")?;
    Proton::detect(&arma)
}
