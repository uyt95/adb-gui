package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import models.Device
import models.InstallMode
import util.MessageHelper
import util.ExecuteHelper

object InstallService {
    fun install(device: Device, path: String, mode: InstallMode, callback: (success: Boolean) -> Unit) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                when (mode) {
                    InstallMode.NORMAL -> installNormal(device, path, callback)
                    InstallMode.PRIV_APP -> installPrivApp(device, path, callback)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
                callback.invoke(false)
            }
        }
    }

    private fun installNormal(device: Device, path: String, callback: (success: Boolean) -> Unit) {
        val response = ExecuteHelper.executeAdb(listOf("install", path), device)
        if (!response.trim().endsWith("Success")) {
            throw Throwable(response)
        }
        callback.invoke(true)
    }

    private fun installPrivApp(device: Device, path: String, callback: (success: Boolean) -> Unit) {
        var response = ExecuteHelper.executeAdb(listOf("root"), device).trim()
        if (response != "restarting adbd as root" && response != "adbd is already running as root") {
            throw Throwable(response)
        }
        response = ExecuteHelper.executeAdb(listOf("remount"), device).trim()
        if (response != "remount succeeded") {
            throw Throwable(response)
        }
        response = ExecuteHelper.executeAdb(listOf("push", path, "/system/priv-app"), device).trim()
        if (!response.startsWith("$path: 1 file pushed, 0 skipped.")) {
            throw Throwable(response)
        }
        callback.invoke(true)
    }
}