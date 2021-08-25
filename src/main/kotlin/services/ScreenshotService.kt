package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import models.Device
import util.ExecuteHelper
import util.MessageHelper
import java.io.File

object ScreenshotService {
    private const val adbGuiDir = "/sdcard/adb-gui"
    private const val tempScreenshot = "tmp-screenshot.png"

    fun takeScreenshot(device: Device, path: String, callback: (success: Boolean) -> Unit) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                createDirectoryOnDevice(device)
                createScreenshot(device)
                downloadScreenshot(device, path)
                deleteTmpScreenshot(device)
                callback.invoke(true)
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
                callback.invoke(false)
            }
        }
    }

    private fun createDirectoryOnDevice(device: Device) {
        val response = ExecuteHelper.executeAdb(listOf("shell", "mkdir", "-p", adbGuiDir), device)
        if (response.isNotEmpty()) {
            throw Throwable("Failed to create adb-gui directory: $response")
        }
    }

    private fun createScreenshot(device: Device) {
        val response = ExecuteHelper.executeAdb(listOf("shell", "screencap", "-p", "$adbGuiDir/$tempScreenshot"), device)
        if (response.isNotEmpty()) {
            throw Throwable("Failed to create screenshot: $response")
        }
    }

    private fun downloadScreenshot(device: Device, path: String) {
        val response = ExecuteHelper.executeAdb(listOf("pull", "$adbGuiDir/$tempScreenshot", path), device)
        if (response.startsWith("adb: error:") || !File(path).exists()) {
            throw Throwable("Failed to download screenshot: $response")
        }
    }

    private fun deleteTmpScreenshot(device: Device) {
        ExecuteHelper.executeAdb(listOf("shell", "rm", "$adbGuiDir/$tempScreenshot"), device)
    }
}