
# pamm UI

Flutter-based user interface for **pamm** (Personal ARMA Mod Manager).

## Overview

This directory contains the desktop application UI for `pamm`. It allows users to browse, install, update, and manage their ARMA 3 mod packs within their community mod repositories. The UI communicates with the core Rust library via `flutter_rust_bridge`.

## Getting Started

To develop or build the UI, you will need [Flutter](https://docs.flutter.dev/get-started/install) installed on your system.

### Dependencies

- Flutter SDK (latest stable recommended)
- Rust toolchain (for building the core library)
- `flutter_rust_bridge_codegen` (for generating Dart/Rust bindings)

### Running

To run the application locally for development:

```bash
cd ui
flutter run
```

### Building

To build a release version of the UI:

```bash
cd ui
flutter build windows # or linux
```

For more detailed information on Flutter development, see the [online documentation](https://docs.flutter.dev/).
