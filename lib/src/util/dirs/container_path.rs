use anyhow::Context;
use std::path::Path;

/// Renders a container-visible Unix path the way Arma's Windows binary needs to
/// see it: the container root is mounted as `Z:`, so the path becomes `Z:` plus
/// the same path with `/` swapped for `\`.
///
/// The input must already be a *container* path — see
/// [`crate::util::dirs::steam_install::SteamInstall::container_path`]. Passing a
/// host path works fine on native Steam and silently points at the wrong
/// directory under Flatpak.
pub fn to_arma_path(container: &Path) -> anyhow::Result<String> {
    let path = container
        .to_str()
        .with_context(|| format!("mod paths must be UTF-8, got {container:#?}"))?;

    Ok(format!("Z:{}", path.replace('/', "\\")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_the_container_root_onto_z_drive() {
        assert_eq!(
            to_arma_path(Path::new("/mnt/games/FPArma/@ace")).unwrap(),
            r"Z:\mnt\games\FPArma\@ace"
        );
    }

    // The `@` is part of the folder name, not a separator, and spaces are legal.
    #[test]
    fn leaves_everything_but_the_separators_alone() {
        assert_eq!(
            to_arma_path(Path::new("/home/bob/My Mods/@CBA_A3")).unwrap(),
            r"Z:\home\bob\My Mods\@CBA_A3"
        );
    }
}
