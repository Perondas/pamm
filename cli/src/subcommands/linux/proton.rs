use anyhow::Context;
use pamm_lib::util::dirs::arma_install::find_arma_install;
use pamm_lib::util::proton::proton_env::Proton;

/// Arma's Proton setup, detected fresh.
pub(super) fn detect_proton() -> anyhow::Result<Proton> {
    let arma = find_arma_install().context("Failed to find the Arma 3 installation")?;
    Proton::detect(&arma)
}
