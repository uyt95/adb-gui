package services

import java.util.prefs.Preferences

object SettingsService {
    private val preferences = Preferences.userRoot().node("settings")

    private const val KEY_ADB_PATH = "adb-path"
    private const val KEY_EMULATOR_PATH = "emulator-path"

    var adbPath: String
        get() {
            return preferences.get(KEY_ADB_PATH, "${System.getProperty("user.home")}/Android/Sdk/platform-tools/adb")
        }
        set(value) {
            preferences.put(KEY_ADB_PATH, value)
        }

    var emulatorPath: String
        get() {
            return preferences.get(KEY_EMULATOR_PATH, "${System.getProperty("user.home")}/Android/Sdk/emulator/emulator")
        }
        set(value) {
            preferences.put(KEY_EMULATOR_PATH, value)
        }
}