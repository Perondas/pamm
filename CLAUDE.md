# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

pamm (Personal ARMA Mod Manager) is an ARMA 3 mod manager for self-hosted community mod repositories. Key differentiators vs. other Arma mod managers (and constraints to keep in mind):

- A repo can host **multiple packs** with **parent/child relationships** (e.g. an "event" pack depends on a "core" pack).
- Downloads use **delta patching of PBO files** via HTTP multi-range requests — `bi_fs_rs` reads PBO part offsets and only the changed parts are fetched. Any server work must preserve multi-range support (tested against caddy).
- Both client and server keep checksum caches to avoid re-indexing unchanged files. `sled` is used for the on-disk KV cache.
- **Not compatible with other mod managers** — pamm owns the layout and metadata files in the repo directory.

## Workspace layout

This is a Cargo workspace with three Rust crates plus a Flutter app:

- `lib/` (`pamm_lib`) — core domain logic. Everything the CLI and UI do lives here.
- `cli/` (`pamm_cli`) — thin clap-based CLI on top of `pamm_lib`.
- `ui/rust/` (`rust_lib_ui`) — `flutter_rust_bridge` shim that re-exports `pamm_lib` functions to Dart. Built as `cdylib`/`staticlib`.
- `ui/` — Flutter desktop app (Linux + Windows). Calls into `rust_lib_ui` through generated bindings (`ui/lib/src/rust/`).
- `fpa/` — a real example repo (a checked-in sample, not test fixtures) showing the on-disk format: `repo.config.json`, one `<pack>.pack.config.json` + `<pack>.pack.settings.json` per pack, and `<pack>_pack_addons/` directories.

## Architecture of `pamm_lib`

The crate is split into four top-level modules. Understanding the split matters because of how things are wired together:

### `models/` — pure data + diff logic
- `repo/`: `RepoConfig` (named, holds a set of pack names), `RepoUserSettings` (client-only, stores the remote URL).
- `pack/`: `PackConfig` (server-defined: name, parent, addons map, client params, servers), `PackUserSettings` (client-only: enabled optionals, launch params), `PackDiff`, `ServerInfo`.
- `index/`: `ChecksumIndex` (flat addon → checksum, for quick-checking without a full tree walk), the recursive `IndexNode` tree (with `PBOPart` byte ranges for delta patching), `DiffIndex` and `NodeDiff` (the diff/patch model — `FileModification::PBO` carries `new_order`, `required_checksums`, byte offsets so the patcher can reconstruct a target file).

### `io/` — everything that touches the filesystem or network
- `serialization/`: Two formats, selected per-type via macros — `hr_serializable!(T)` for human-readable JSON (configs/settings) and `bin_serializable!(T)` for bincode (indexes and other internal files). Each type's "known" filename is declared via `known_file_name!(T, "name.ext")` in `io/known_file.rs`. To add a new persisted type, declare both.
- `fs/`: `KnownFSReadable`/`NamedFSReadable`/`KnownFSWritable` traits drive reads/writes from the repo path. `fs/cache/kv_cache.rs` wraps sled and bincode for the on-disk file checksum cache. `fs/pack/index_generator.rs` builds an index from disk; `fs/pack/pbo_reader.rs` extracts PBO part metadata via `bi_fs_rs`.
- `net/`: `Downloadable` trait, `index_downloader`, `remote_patcher` (the multi-range PBO patcher — opens a temp file `<file>.pamm.temp`, streams patched parts, then replaces).
- `progress_reporting/`: `ProgressReporter` trait — both the CLI (`indicatif`) and the UI (custom Dart stream) implement it.

### `handle/` — the API surface (`RepoHandle`)
`RepoHandle::open(&Path)` is the entry point: it loads `RepoConfig` and optional `RepoUserSettings`. Operations are added as **trait impls organized by capability**, not methods on the struct directly:

- `handle/reading/` — `GetPack`, `GetRepoInfo`, `GetAddonPaths`, `GetLinuxAddonPaths`, etc. Each capability is a trait so it can be mocked.
- `handle/writing/` — `AddPack`, `update_pack`, `apply_diff`, `delete_pack`, `save_pack_settings`, `update_repo_config`.
- `handle/actions/` — higher-level orchestrations: `sync/` (diff → apply), `launch/steam.rs` (builds a `steam://rungameid/107410//` URL with `-mod=...` and shells out via `open`).
- `handle/externals/`, `handle/optionals/` — managing addons that live outside packs / optional addons toggled by user settings.
- `handle/mock_handle.rs` (test-only) — `mockall` mock of the trait set, with a `MockHandleExt` helper for setting up fake packs in tests.

When adding new repo operations, follow the existing pattern: define a trait in the appropriate `handle/` submodule, `impl` it for `RepoHandle`, and add the mock impl to `mock_handle.rs` if it's reachable from code that has tests.

### `util/` — small helpers
`iterator_diff`, `test_utils::TestTempDir` (used by integration tests — they hit the real filesystem, not mocked), Linux-only Steam VDF path resolution (`util/linux/`, behind `cfg(target_os = "linux")`).

## Platform-specific code

Linux and non-Linux diverge in two specific places:

- `handle/reading/get_linux_addon_paths.rs` is `#[cfg(target_os = "linux")]`; `handle/actions/launch/steam.rs` uses `cfg_select!` to pick between `get_linux_addon_paths` (Linux) and `get_canonical_addon_paths` (Windows). The Linux launcher must put the load path inside the Arma install directory — don't change this without testing the Steam launch flow.
- `util/linux/get_arma_install_dir.rs` parses Steam library VDFs via `steam-vdf-parser` (Linux-only dep in `lib/Cargo.toml`).

## Common commands

The workspace uses Rust edition 2024 (lib + cli) and 2021 (ui/rust). Use the stable toolchain — CI installs via `actions-rust-lang/setup-rust-toolchain@v1`.

```bash
# Build everything (run from repo root)
cargo build
cargo build --release

# Test the whole workspace
cargo test

# Test just the lib crate
cargo test -p pamm_lib

# Run a single test (substring match)
cargo test -p pamm_lib test_remove_disabled_optionals

# Lint (CI runs this with -D warnings — match it locally before pushing)
cargo clippy -- -D warnings

# Run the CLI in-place
cargo run -p pamm_cli -- --help
cargo run -p pamm_cli -- --log-level debug sync <pack>
```

### Flutter UI

Requires Flutter stable + the Rust toolchain. `flutter_rust_bridge_codegen` generates the Dart bindings from `ui/rust/src/api/`; the config is in `ui/flutter_rust_bridge.yaml` (`rust_input: crate::api`, `dart_output: lib/src/rust`).

```bash
cd ui
flutter pub get
flutter pub run build_runner build --delete-conflicting-outputs   # generates freezed/json_serializable code
flutter_rust_bridge_codegen generate                              # regenerate Dart↔Rust bindings after changing ui/rust/src/api/
flutter run                                                       # dev run
flutter build linux   # or: flutter build windows
```

When changing the FFI surface, edit files under `ui/rust/src/api/` and regenerate — do **not** hand-edit `ui/rust/src/frb_generated.rs` or `ui/lib/src/rust/`.

## Conventions worth knowing

- **Errors:** everything returns `anyhow::Result`. Use `.context(...)` / `ensure!` / `anyhow::bail!` consistently with surrounding code.
- **Logging:** the `log` crate. CLI configures `env_logger` from `--log-level`; the UI ships its own log forwarding (`ui/rust/src/api/logging.rs` → `services/rust_log_service.dart`).
- **Tests live next to code** in `#[cfg(test)] mod tests` blocks. Integration-style tests use `TestTempDir` from `util/test_utils.rs` and write real files. Unit tests of `handle/` traits use `MockHandle` from `mock_handle.rs`.
- **Filenames are centralized.** Don't hardcode `"repo.config.json"` etc. — declare via `known_file_name!` and use `T::file_name()` / the `Named*` traits. Pack-addon directory names go through `io/name_consts::get_pack_addon_directory_name`.
- **Don't bypass the diff/patch path** when modifying files in a repo addon dir during sync — the temp-file dance in `remote_patcher.rs` is what keeps partial updates safe.

## Release flow

Tags matching `v[0-9].[0-9].[0-9]` trigger `.github/workflows/release.yml`, which builds the CLI (Linux + Windows binaries, `.deb`, `.rpm`) and the UI (Linux + Windows bundles, `.deb`/`.rpm` via fastforge, Windows `.exe` via Inno Setup). The version is patched into `cli/Cargo.toml` and `ui/pubspec.yaml` from the tag name at build time — don't bump versions in source for a release.
