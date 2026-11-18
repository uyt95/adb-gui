# Built by .github/workflows/rpm.yml. Cargo fetches crates from crates.io
# during %build, so the build needs network access (not suitable for Koji).
%global app_id io.github.uyt95.AdbGui
%global debug_package %{nil}

Name:           adb-gui
Version:        %{?version_override}%{!?version_override:3.0.0}
Release:        1%{?dist}
Summary:        A user interface on top of adb (Android Debug Bridge)
License:        MIT
URL:            https://github.com/uyt95/adb-gui
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo >= 1.92
BuildRequires:  rust >= 1.92
BuildRequires:  gcc
BuildRequires:  pkgconfig(gtk4) >= 4.12
BuildRequires:  pkgconfig(libadwaita-1) >= 1.5
BuildRequires:  desktop-file-utils

Requires:       gtk4 >= 4.12
Requires:       libadwaita >= 1.5
Recommends:     android-tools

%description
ADB GUI is a GTK interface on top of adb (Android Debug Bridge). It manages
remote connections, starts emulators, sends keyboard input, offers an Android
TV remote control, installs APKs, runs automations, manages forward and
reverse proxies and takes screenshots.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release --locked

%install
install -Dm0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dm0644 data/%{app_id}.desktop %{buildroot}%{_datadir}/applications/%{app_id}.desktop
install -Dm0644 data/icons/hicolor/scalable/apps/%{app_id}.svg \
    %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/%{app_id}.svg

%check
desktop-file-validate %{buildroot}%{_datadir}/applications/%{app_id}.desktop
cargo test --release --locked

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/applications/%{app_id}.desktop
%{_datadir}/icons/hicolor/scalable/apps/%{app_id}.svg

%changelog
* Wed Nov 18 2026 uyt95 <uyt95@users.noreply.github.com> - 3.0.0-1
- Rewrite in Rust with GTK 4 and libadwaita
