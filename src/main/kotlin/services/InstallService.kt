package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import models.Device
import util.ErrorHelper
import util.ExecuteHelper

object InstallService {
    fun install(device: Device, path: String, callback: (success: Boolean) -> Unit) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf("install", path), device)
                if (!response.trim().endsWith("Success")) {
                    throw Throwable(response)
                }
                callback.invoke(true)
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
                callback.invoke(false)
            }
        }
    }
}