# pamm

Personal ARMA Mod Manager

## Overview

pamm is another ARMA 3 mod manager for communities that host their own mods.

It is designed to be lightweight and fast, while still providing all the features you need to manage your mod
packs.

It is NOT compatible with other mod managers.

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
    - Linux
- GUI     

## Planned Features
Are tracked in the issues tab

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
