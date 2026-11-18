# ADB-GUI

A user interface on top of the adb (Android Debug Bridge), written in Rust with GTK 4 and libadwaita.

## Features
* Save and manage remote connections
* Start emulators
* Send keyboard input (Ctrl+Enter sends the text, Ctrl+Backspace and Ctrl+Tab send those keys)
* (Android TV) remote control
* Install apks (normal or as priv-app), with drag and drop
* Automations (chain adb commands to automate a multi-step action)
* Save and manage forward and reverse proxies
* Take screenshots

The interface is in English or Dutch, following the `LANG` / `LC_MESSAGES` locale.

## Requirements
* GTK 4.12 or newer and libadwaita 1.5 or newer (Ubuntu 24.04, Fedora 40 or newer)
* `adb` and, for emulators, the Android SDK `emulator` binary. Their paths are detected from `PATH`
  or `~/Android/Sdk` and can be changed on the Settings page.

Build dependencies on Debian/Ubuntu:

```sh
sudo apt install build-essential libgtk-4-dev libadwaita-1-dev
```

On Fedora:

```sh
sudo dnf install gtk4-devel libadwaita-devel
```

## Building and running
```sh
cargo run              # debug build
cargo build --release  # binary at target/release/adb-gui
cargo test             # unit tests (no device needed)
```

## Data
Connections, emulator parameters, proxies, automations and settings are stored in
`~/.local/share/adb-gui/data.json` (or `$XDG_DATA_HOME/adb-gui/data.json`).
