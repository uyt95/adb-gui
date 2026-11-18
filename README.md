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

## Packaging
* `data/io.github.uyt95.AdbGui.desktop` and the icon in `data/icons/` are the desktop integration files.
* `packaging/adb-gui.spec` builds an RPM. The `RPM` GitHub Actions workflow builds it on Fedora for every
  push and pull request and uploads it as a build artifact. For `v*` tags it also attaches the RPM to the
  GitHub release.

To build the RPM locally on Fedora:

```sh
sudo dnf install rpm-build cargo rust gcc gtk4-devel libadwaita-devel desktop-file-utils
mkdir -p ~/rpmbuild/SOURCES
git archive --prefix=adb-gui-3.0.0/ -o ~/rpmbuild/SOURCES/adb-gui-3.0.0.tar.gz HEAD
rpmbuild -ba packaging/adb-gui.spec
```
