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

## Planned Features

- Gui
- Some sort of mod pack documentation system
    - Markdown support?
- Better delta patching (support for more file types, or just larger files in general)
- Server lists
- HTTP authentication support
- Usage of locally installed mods outside the ones managed
- macOS support (untested at the moment)
- Launching the ARMA binary directly from pamm 

## Notes

Has only been tested using caddy as a web server. It should work with any web server that supports multiple range
requests.