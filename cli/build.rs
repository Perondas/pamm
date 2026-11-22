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

    // OUT_DIR is target/<profile>/build/pamm_cli-<hash>/out, or
    // target/<profile>/build/pamm_cli/<hash>/out with cargo's newer build-dir
    // layout. Climb to the `build` directory instead of counting levels.
    let man_dir = out_dir
        .ancestors()
        .find(|p| p.file_name().is_some_and(|n| n == "build"))
        .and_then(|build| build.parent())
        .ok_or(std::io::ErrorKind::NotFound)?
        .join("man");

    // Start clean, so pages of removed commands do not linger.
    if man_dir.exists() {
        std::fs::remove_dir_all(&man_dir)?;
    }
    std::fs::create_dir_all(&man_dir)?;
    clap_mangen::generate_to(Args::command(), &man_dir)
}
