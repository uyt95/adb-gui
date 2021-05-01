package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import util.ErrorHelper
import util.execute

object KeyboardService {
    fun sendBackspaceKey() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                execute(SettingsService.adbPath, listOf("shell", "input", "keyevent", "KEYCODE_DEL"))
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            }
        }
    }

    fun sendTabKey() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                execute(SettingsService.adbPath, listOf("shell", "input", "keyevent", "KEYCODE_TAB"))
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            }
        }
    }

    fun sendText(text: String) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                execute(SettingsService.adbPath, listOf("shell", "input", "keyboard", "text", "'$text'"))
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            }
        }
    }
}