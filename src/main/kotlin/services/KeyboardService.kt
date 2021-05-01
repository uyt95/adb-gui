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
                val response = execute(SettingsService.adbPath, listOf("shell", "input", "keyevent", "KEYCODE_DEL"))
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            }
        }
    }

    fun sendTabKey() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = execute(SettingsService.adbPath, listOf("shell", "input", "keyevent", "KEYCODE_TAB"))
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            }
        }
    }

    fun sendText(text: String) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = execute(SettingsService.adbPath, listOf("shell", "input", "keyboard", "text", "'$text'"))
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            }
        }
    }
}