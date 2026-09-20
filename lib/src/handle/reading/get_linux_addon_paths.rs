use crate::handle::addons::ResolveAddons;
use crate::handle::externals::get_external_addons_paths::GetExternalAddonsPaths;
use crate::handle::optionals::GetOptionalsPaths;
use crate::handle::reading::get_pack::GetPack;
use crate::handle::reading::get_repo_info::GetRepoInfo;
use crate::io::fs::util::symlink::{create_or_recreate_symlink, relative_path};
use crate::util::dirs::arma_install::find_arma_install;
use crate::util::dirs::flatpak::{GrantState, grant_command, steam_grant_state};
use crate::util::dirs::steam_install::SteamInstall;
use anyhow::{Context, anyhow, bail, ensure};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub trait GetLinuxAddonPaths {
    /// Gets the paths to all enabled addons of the pack as relative path strings
    /// of the form `pamm/<repo name>/<addon>` or `pamm/externals/<addon>`,
    /// relative to the Arma install directory (where the `pamm` symlinks are
    /// created), suitable for use as `-mod=` launch parameters.
    ///
    /// The paths are relative because the game may well be looking at them from
    /// inside a sandbox, where the Arma install directory has a different
    /// absolute path than it does here.
    fn get_linux_addon_paths(
        &self,
        pack_name: &str,
        disable_optionals: bool,
    ) -> anyhow::Result<Vec<String>>;
}

impl<T> GetLinuxAddonPaths for T
where
    T: GetPack + GetRepoInfo + GetExternalAddonsPaths,
{
    fn get_linux_addon_paths(
        &self,
        pack_name: &str,
        disable_optionals: bool,
    ) -> anyhow::Result<Vec<String>> {
        log::debug!("Resolving addon paths for pack '{}'", pack_name);

        let arma = find_arma_install().context(anyhow!("Failed to get Arma installation"))?;
        log::debug!(
            "Found Arma installation at {:?} ({:?} Steam)",
            arma.install_dir,
            arma.steam.flavour
        );

        let pamm_dir = arma.install_dir.join("pamm");
        let externals_dir = pamm_dir.join("externals");

        log::trace!(
            "Ensuring pamm directories exist at {:?} and {:?}",
            pamm_dir,
            externals_dir
        );
        create_dir_all(&pamm_dir)
            .context(anyhow!("Failed to create PAMM directory at {:?}", pamm_dir))?;
        create_dir_all(&externals_dir).context(anyhow!(
            "Failed to create externals directory at {:?}",
            externals_dir
        ))?;

        // Canonical, because the link we write has to be computed against the
        // directory the repo really lives in, not a relative or symlinked
        // spelling of it.
        let repo_path = self.get_repo_path().canonicalize().context(anyhow!(
            "Failed to resolve repo path {:?}",
            self.get_repo_path()
        ))?;

        let symlink_path = pamm_dir.join(&self.get_config().name);
        log::debug!("Creating repo symlink for pack: {:?}", symlink_path);

        link_into_game(&arma.steam, &repo_path, &symlink_path)?;

        let required_addons = self.resolve_addons(pack_name)?;
        let optional_addons = if !disable_optionals {
            self.get_optional_paths(pack_name)?
        } else {
            vec![]
        };

        let mut addons = required_addons
            .iter()
            .chain(&optional_addons)
            .map(|p| {
                p.to_str()
                    .map(|s| s.to_string())
                    .context(anyhow!("Failed to convert path to string: {:?}", p))
            })
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .map(|rel| format!("pamm/{}/{}", self.get_config().name, rel))
            .collect::<Vec<_>>();

        let externals: Vec<String> = self
            .get_external_addon_paths(pack_name)
            .context(anyhow!("Failed to read external addons"))?;

        log::debug!("Found {} external addons", externals.len());

        for external in externals {
            let external_path = Path::new(&external);
            if let Some(folder_name) = external_path.file_name() {
                let link_path = externals_dir.join(folder_name);
                log::trace!(
                    "Processing external addon {:?} -> {:?}",
                    external_path,
                    link_path
                );
                link_into_game(&arma.steam, external_path, &link_path)?;
                if let Some(folder_name_str) = folder_name.to_str() {
                    addons.push(format!("pamm/externals/{}", folder_name_str));
                }
            } else {
                log::warn!(
                    "Failed to determine folder name for external addon {:?}",
                    external_path
                );
            }
        }

        log::debug!("Successfully resolved {} total addon paths", addons.len());

        Ok(addons)
    }
}

/// How a link into the Arma install has to be spelled.
#[derive(Debug, PartialEq)]
enum LinkTarget {
    /// Resolves both for us and for the game, so we can check that it does.
    Shared(PathBuf),
    /// Crosses Steam's `$HOME` remap, so it can only be written the way Steam
    /// sees it, and dangles on this side by design.
    SandboxOnly(PathBuf),
}

impl LinkTarget {
    fn path(&self) -> &Path {
        match self {
            LinkTarget::Shared(path) | LinkTarget::SandboxOnly(path) => path,
        }
    }
}

/// Links `target` into the Arma install so that the game can load addons from it.
///
/// Two things have to line up before the game can follow such a link. The target
/// has to be reachable from inside Steam's sandbox at all, and the link has to be
/// written in a spelling that resolves to the same directory on both sides of
/// that sandbox.
fn link_into_game(steam: &SteamInstall, target: &Path, link: &Path) -> anyhow::Result<()> {
    ensure_reachable(steam, target)?;

    let link_dir = link
        .parent()
        .context(anyhow!("Symlink path {:?} has no parent directory", link))?;

    // Both sides have to be absolute for the relative computation below to mean
    // anything. They are: the install directory comes from Steam's own absolute
    // library path, and every target is canonicalized before it gets here.
    ensure!(
        link_dir.is_absolute() && target.is_absolute(),
        "Cannot link {:?} into {:?}: both paths must be absolute",
        target,
        link_dir
    );

    let link_target = link_target_for(steam, link_dir, target);

    create_or_recreate_symlink(link_target.path(), link).context(anyhow!(
        "Failed to link {:?} into the Arma install at {:?}",
        target,
        link
    ))?;

    if let LinkTarget::Shared(path) = &link_target {
        ensure!(
            link.exists(),
            "Symlink {:?} -> {:?} does not resolve to anything",
            link,
            path
        );
    }

    Ok(())
}

/// Picks the spelling for a link at `link_dir` pointing at `target`.
///
/// A relative target never names the directory Steam's sandbox remaps, so it
/// resolves identically for us and for the game -- as long as link and target sit
/// on the same side of that remap. When they do not, no relative path can span
/// the two and only Steam's own spelling of the target will resolve inside the
/// sandbox.
fn link_target_for(steam: &SteamInstall, link_dir: &Path, target: &Path) -> LinkTarget {
    if steam.is_inside_sandbox_home(link_dir) == steam.is_inside_sandbox_home(target) {
        LinkTarget::Shared(relative_path(link_dir, target).unwrap_or_else(|| target.to_path_buf()))
    } else {
        LinkTarget::SandboxOnly(steam.to_steam_path(target))
    }
}

/// Refuses to build a link the game could never follow.
///
/// Flatpak Steam cannot see the real home directory, so a repo outside its data
/// directory is invisible to Arma until the user grants access to it. Failing
/// here beats launching into a game that silently loads no mods.
fn ensure_reachable(steam: &SteamInstall, target: &Path) -> anyhow::Result<()> {
    if !steam.requires_flatpak_grant(target) {
        return Ok(());
    }

    match steam_grant_state(target) {
        GrantState::Granted => {
            log::debug!("Flatpak Steam has been granted access to {target:?}");
            Ok(())
        }
        // Better a launch that might work than a refusal based on a guess.
        GrantState::Unknown => {
            log::warn!(
                "Could not tell whether Flatpak Steam can access {target:?}. \
                 If Arma starts without its mods, run:\n  {}",
                grant_command(target)
            );
            Ok(())
        }
        GrantState::Missing => {
            let sandbox_home = steam
                .sandbox_home()
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            bail!(
                "{} is outside Steam's Flatpak sandbox, so Arma cannot load anything from it.\n\
                 Either keep it below {}, or grant Steam access to it once:\n  {}",
                target.display(),
                sandbox_home,
                grant_command(target)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::dirs::steam_install::{SteamFlavour, flatpak_app_dir};
    use crate::util::test_utils::TestTempDir;
    use std::fs::create_dir_all;

    const HOME: &str = "/home/user";

    fn steam(flavour: SteamFlavour) -> SteamInstall {
        let home = PathBuf::from(HOME);
        let root = match flavour {
            SteamFlavour::Flatpak => flatpak_app_dir(&home).join(".local/share/Steam"),
            _ => home.join(".local/share/Steam"),
        };
        SteamInstall::new(root, flavour, home)
    }

    fn link_target(steam: &SteamInstall, link_dir: &str, target: &str) -> LinkTarget {
        link_target_for(steam, Path::new(link_dir), Path::new(target))
    }

    // Nothing is remapped, so the link is plain relative.
    #[test]
    fn a_native_install_gets_a_relative_link() {
        assert_eq!(
            link_target(
                &steam(SteamFlavour::Native),
                "/home/user/.local/share/Steam/steamapps/common/Arma 3/pamm",
                "/home/user/pamm_repo"
            ),
            LinkTarget::Shared(PathBuf::from("../../../../../../../pamm_repo"))
        );
    }

    // A repo kept inside Flatpak Steam's data directory is the case that works
    // without any setup, and a relative link is what makes it work: the same
    // link text resolves under `~/.var/app/...` for us and under `$HOME` for the
    // game.
    #[test]
    fn a_repo_inside_the_flatpak_data_dir_gets_a_relative_link() {
        let steam = steam(SteamFlavour::Flatpak);
        let app_dir = flatpak_app_dir(Path::new(HOME));

        let target = link_target(
            &steam,
            &app_dir
                .join(".local/share/Steam/steamapps/common/Arma 3/pamm")
                .to_string_lossy(),
            &app_dir.join("pamm_repo").to_string_lossy(),
        );

        assert_eq!(
            target,
            LinkTarget::Shared(PathBuf::from("../../../../../../../pamm_repo"))
        );
        // And that is exactly what the game, looking at the sandbox's spelling of
        // the same two directories, would need the link to say.
        assert_eq!(
            relative_path(
                Path::new("/home/user/.local/share/Steam/steamapps/common/Arma 3/pamm"),
                Path::new("/home/user/pamm_repo")
            ),
            Some(PathBuf::from("../../../../../../../pamm_repo"))
        );
    }

    // A repo in the real home is on the other side of the remap. A relative link
    // would be computed from the `~/.var/app` prefix and land somewhere else
    // entirely inside the sandbox, so the link has to be absolute instead.
    #[test]
    fn a_repo_outside_the_flatpak_data_dir_gets_an_absolute_link() {
        let steam = steam(SteamFlavour::Flatpak);
        let app_dir = flatpak_app_dir(Path::new(HOME));

        assert_eq!(
            link_target(
                &steam,
                &app_dir
                    .join(".local/share/Steam/steamapps/common/Arma 3/pamm")
                    .to_string_lossy(),
                "/home/user/git/pamm_repo"
            ),
            LinkTarget::SandboxOnly(PathBuf::from("/home/user/git/pamm_repo"))
        );
    }

    // The mirror image: the game sits in a library on another drive, outside the
    // remap, while the repo is inside it. The link then has to name the repo the
    // way Steam sees it, which is not the path we opened it by.
    #[test]
    fn a_link_from_outside_the_flatpak_data_dir_uses_steams_spelling() {
        let steam = steam(SteamFlavour::Flatpak);
        let app_dir = flatpak_app_dir(Path::new(HOME));

        assert_eq!(
            link_target(
                &steam,
                "/mnt/games/SteamLibrary/steamapps/common/Arma 3/pamm",
                &app_dir.join("pamm_repo").to_string_lossy(),
            ),
            LinkTarget::SandboxOnly(PathBuf::from("/home/user/pamm_repo"))
        );
    }

    // End to end on a real (native) layout: the link has to be written relative
    // and it has to resolve to the repo.
    #[test]
    fn linking_a_repo_into_the_game_writes_a_relative_link_that_resolves() {
        let tmp = TestTempDir::new("pamm_link_into_game_relative");
        let root = tmp.path().canonicalize().unwrap();
        let pamm_dir = root.join("Steam/steamapps/common/Arma 3/pamm");
        let repo = root.join("pamm_repo");
        create_dir_all(&pamm_dir).unwrap();
        create_dir_all(repo.join("core/addons/@addon")).unwrap();

        let steam = SteamInstall::new(root.join("Steam"), SteamFlavour::Native, root.clone());
        let link = pamm_dir.join("repo");

        link_into_game(&steam, &repo, &link).unwrap();

        assert_eq!(
            std::fs::read_link(&link).unwrap(),
            PathBuf::from("../../../../../pamm_repo")
        );
        // Which is the whole point: `-mod=pamm/repo/core/addons/@addon` has to
        // reach the addon through it.
        assert!(link.join("core/addons/@addon").is_dir());
    }

    // Relaunching after the repo moved must not trip over the link left behind.
    #[test]
    fn linking_replaces_a_link_from_an_earlier_launch() {
        let tmp = TestTempDir::new("pamm_link_into_game_replaces");
        let root = tmp.path().canonicalize().unwrap();
        let pamm_dir = root.join("Steam/steamapps/common/Arma 3/pamm");
        let repo = root.join("moved/pamm_repo");
        create_dir_all(&pamm_dir).unwrap();
        create_dir_all(&repo).unwrap();

        let link = pamm_dir.join("repo");
        std::os::unix::fs::symlink(root.join("old_repo"), &link).unwrap();

        let steam = SteamInstall::new(root.join("Steam"), SteamFlavour::Native, root.clone());
        link_into_game(&steam, &repo, &link).unwrap();

        assert_eq!(
            std::fs::read_link(&link).unwrap(),
            PathBuf::from("../../../../../moved/pamm_repo")
        );
    }
}
