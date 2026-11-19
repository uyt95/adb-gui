//! APK install modes.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstallMode {
    Normal,
    PrivApp,
}

impl InstallMode {
    pub const ALL: [InstallMode; 2] = [InstallMode::Normal, InstallMode::PrivApp];

    pub fn label(self) -> &'static str {
        match self {
            InstallMode::Normal => "Normal",
            InstallMode::PrivApp => "Priv app",
        }
    }
}
