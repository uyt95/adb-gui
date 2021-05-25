package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import models.Device
import util.MessageHelper
import util.ExecuteHelper

object KeyboardService {
    fun sendBackspaceKey(device: Device) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf("shell", "input", "keyevent", "KEYCODE_DEL"), device)
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            }
        }
    }

    fun sendTabKey(device: Device) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf("shell", "input", "keyevent", "KEYCODE_TAB"), device)
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            }
        }
    }

    fun sendText(device: Device, text: String, successCallback: () -> Unit) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf("shell", "input", "keyboard", "text", "'$text'"), device)
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
                successCallback.invoke()
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            }
        }
    }
}