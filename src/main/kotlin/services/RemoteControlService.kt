package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import models.Device
import models.RemoteControlButton
import util.MessageHelper
import util.ExecuteHelper

object RemoteControlService {
    fun sendButtonPress(device: Device, button: RemoteControlButton) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf("shell", "input", "keyevent", "KEYCODE_${button.key}"), device)
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            }
        }
    }
}