//! User settings.

use serde::{Deserialize, Serialize};

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingKey {
    AdbPath,
    EmulatorPath,
    ScreenshotPath,
}

impl SettingKey {
    pub const ALL: [SettingKey; 3] = [
        SettingKey::AdbPath,
        SettingKey::EmulatorPath,
        SettingKey::ScreenshotPath,
    ];
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub adb_path: String,
    pub emulator_path: String,
    pub screenshot_path: String,
}

impl Settings {
    pub fn get(&self, key: SettingKey) -> &str {
        match key {
            SettingKey::AdbPath => &self.adb_path,
            SettingKey::EmulatorPath => &self.emulator_path,
            SettingKey::ScreenshotPath => &self.screenshot_path,
        }
    }

    pub fn set(&mut self, key: SettingKey, value: String) {
        match key {
            SettingKey::AdbPath => self.adb_path = value,
            SettingKey::EmulatorPath => self.emulator_path = value,
            SettingKey::ScreenshotPath => self.screenshot_path = value,
        }
    }
}
