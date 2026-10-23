# pamm

Personal ARMA Mod Manager

## Overview

pamm is another ARMA 3 mod manager for communities that host their own mods.

It is designed to be lightweight and fast, while still providing all the features you need to manage your mod
packs.

It is NOT compatible with other mod managers.

## Admin guide

Check the [wiki](https://github.com/Perondas/pamm/wiki) for an admin guode

## Features

- Multiple mod packs in one mod repository
    - All of your community's mod packs can be hosted in one place
    - You can have packs depend on one another (e.g. a base pack with core mods, and event specific packs that depend on the base pack)
- Optional mods
- Delta patching of pbo files
    - Only download what has changed between versions
    - Saves bandwidth for both server and client
- Speed above all
    - Utilize caching to avoid re-indexing files on disk
    - Both server and client side caching
    - Multithreaded indexing and downloading
- Packs are cheap
    - Just a couple files
    - Easy to create and delete
    - Perfect fit for one time use mod packs (like event specific mods)
- Cross-platform
    - Windows
    - Linux, through Steam and Proton (native and Flatpak Steam), see [Linux](#linux)
- GUI     

## Planned Features
Are tracked in the issues tab

## Linux

On Linux, pamm launches the Windows version of Arma 3 through Steam and Proton. Native and Flatpak Steam are both
supported, and Arma can live in any of your Steam libraries.

### One-time Steam setup

Depending on where your mods are stored, Steam may need a little setup before Arma can see them. Run `pamm linux`
inside a repo, or click **Steam setup** on the repo page in the GUI, to see exactly what is needed:

- **Mods in your home directory or a Steam library:** nothing to do.
- **Mods on another drive:** paste the `PRESSURE_VESSEL_FILESYSTEMS_RW=... %command%` line it prints into
  Arma 3's Steam launch options (right-click Arma 3 → Properties → Launch Options).
- **Flatpak Steam with mods outside its sandbox:** run the `flatpak override` command it prints, then restart Steam.

pamm never changes Steam's configuration itself, it only tells you what to set.

### Proton tools

Arma has to be started through Steam once so Proton creates its prefix. After that, pamm can run Windows
programs in the same prefix as the game, using the same Proton version:

```
pamm linux                                # Steam setup, Proton version, and whether TeamSpeak is installed
pamm linux install-teamspeak <installer>  # install TeamSpeak into Arma's prefix (silent, add --interactive for the wizard)
pamm linux teamspeak                      # launch TeamSpeak
pamm linux explorer [path]                # open a file browser in the prefix, e.g. at C:\ or Z:\home
pamm linux kill                           # stop everything in the prefix, including the game
```

Download the **Windows** version of TeamSpeak 3 from the [TeamSpeak downloads page](https://www.teamspeak.com/en/downloads).
The same actions are available in the GUI under the **Proton** button next to the settings button, and the repo page
has a **Launch TeamSpeak** button once it is installed.

pamm finds the Proton version again every time, so switching Arma to a different Proton version in Steam just
works. Arma needs to be launched once after a switch, so Proton can update the prefix.

## Hosting a repo

A pamm repository is split into **source** (what you edit on the server) and **build output** (what clients fetch
over HTTP). The build output lives in a dedicated `www/` subdirectory of the repo and is the only thing the HTTP
server should expose.

```
pamm init                       # create a new empty repo
pamm add-pack                   # register a new pack interactively
# drop addon files into <pack>/addons/@addon_name/ ...
pamm build                      # populate www/ with symlinks + indexes
pamm build <pack>               # rebuild a single pack's subtree
pamm build --copy               # use copies instead of symlinks (Windows / cross-fs)
pamm build --force-refresh      # ignore the on-disk index cache and re-scan everything
```

Each pack lives in its own folder, on the server source, in the build output, and in client repos:

```
<repo>/
  repo.config.json
  server.config.json
  version.pamm                  # repo layout version
  <pack>/
    pack.config.json
    addons/
      @addon_name/...
  www/                          # build output, serve this over HTTP
    repo.config.json
    version.pamm
    <pack>/
      pack.config.json
      addons/@addon_name/...
      indexes/
```

`version.pamm` stores the layout version (a repo without one is treated as version 1, the old flat layout).
Repos with an outdated version are migrated automatically the next time they are opened — both server sources
and client repos. After a server repo migrates, run `pamm build` to regenerate `www/` in the new layout;
clients refuse to sync against a `www/` that has not been rebuilt yet and say so explicitly.

Point your HTTP server's document root at `<repo>/www/`. Stop the HTTP server during a build — the build
overwrites entries in place. Symlink mode is fastest; copy mode is needed when the HTTP server can't follow
symlinks or when running on Windows without Developer Mode.

## Notes

Has only been tested using caddy as a web server. It should work with any web server that supports multiple range
requests.
