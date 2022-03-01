package services

import util.PreferencesRepository

object SettingsService {
    private const val PATH_NAME = "settings"
    private const val KEY_ADB_PATH = "adb-path"
    private const val KEY_DEFAULT_SCREENSHOT_DIRECTORY = "default-screenshot-directory"
    private const val KEY_EMULATOR_PATH = "emulator-path"

    private val repository = PreferencesRepository(PATH_NAME, String::class.java)

    init {
        repository.migrateMapV1ToV2("settings") { oldKey -> oldKey }
    }

    var adbPath: String
        get() = repository.get(KEY_ADB_PATH) ?: "${System.getProperty("user.home")}/Android/Sdk/platform-tools/adb"
        set(value) {
            repository.set(KEY_ADB_PATH, value)
        }

    var defaultScreenshotDirectory: String
        get() = repository.get(KEY_DEFAULT_SCREENSHOT_DIRECTORY) ?: System.getProperty("user.home")
        set(value) {
            repository.set(KEY_DEFAULT_SCREENSHOT_DIRECTORY, value)
        }

    var emulatorPath: String
        get() = repository.get(KEY_EMULATOR_PATH) ?: "${System.getProperty("user.home")}/Android/Sdk/emulator/emulator"
        set(value) {
            repository.set(KEY_EMULATOR_PATH, value)
        }
}