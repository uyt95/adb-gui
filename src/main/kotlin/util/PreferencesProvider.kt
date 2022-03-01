package util

import java.util.prefs.Preferences

object PreferencesProvider {
    fun provide(pathName: String): Preferences {
        return Preferences.userRoot().node(Constants.appDir).node(pathName)
    }

    fun provideV1(pathName: String): Preferences? {
        if (Preferences.userRoot().nodeExists(pathName)) {
            return Preferences.userRoot().node(pathName)
        }
        return null
    }
}