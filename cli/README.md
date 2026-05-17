# pamm CLI

Command-line interface for **pamm** (Personal ARMA Mod Manager).

## Overview

This directory contains the `pamm_cli` binary, a Rust-based command-line interface for managing ARMA 3 mod packs. It provides an efficient and scriptable way to interact with your community mod repositories directly from the terminal, making use of the core `pamm_lib` library.

## Getting Started

To build or run the CLI, you will need the [Rust toolchain](https://rustup.rs/) installed on your system.

### Running

You can compile and run the CLI directly using Cargo:

```bash
cd cli
cargo run -- --help
```

### Building

To build a release version of the CLI:

```bash
cd cli
cargo build --release
```

The compiled binary will be located in the `target/release` directory at the root of the workspace.

## Usage

Use the `--help` flag for a comprehensive list of commands and options:

```bash
pamm_cli --help
```

