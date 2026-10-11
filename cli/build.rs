use clap::CommandFactory;
use pamm_cli_args::args::Args;
use std::path::PathBuf;

/// Renders a man page for every command into `target/<profile>/man`, where
/// the deb and rpm packaging pick them up.
///
/// Writing outside `OUT_DIR` is deliberate: `OUT_DIR` carries a hash, so
/// packaging could not name it.
fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    // OUT_DIR is target/<profile>/build/pamm_cli-<hash>/out.
    let man_dir = out_dir
        .ancestors()
        .nth(3)
        .ok_or(std::io::ErrorKind::NotFound)?
        .join("man");

    // Start clean, so pages of removed commands do not linger.
    if man_dir.exists() {
        std::fs::remove_dir_all(&man_dir)?;
    }
    std::fs::create_dir_all(&man_dir)?;
    clap_mangen::generate_to(Args::command(), &man_dir)
}
