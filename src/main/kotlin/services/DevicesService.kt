package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import models.Device
import util.ErrorHelper
import util.ExecuteHelper

object DevicesService {
    var mainDevicesObserver: ((devices: List<Device>) -> Unit)? = null

    fun refreshDevices() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val devices = ExecuteHelper.executeAdb(listOf("devices")).split("\n")
                    .map { line -> line.split(Regex("\\s+")) }
                    .filter { tokens -> tokens.size == 2 }
                    .map { row ->
                        val idParts = row[0].split(':')
                        val address = if (idParts.size == 2) idParts[0] else row[0]
                        Device(address, row[1])
                    }
                mainDevicesObserver?.invoke(devices)
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
                mainDevicesObserver?.invoke(emptyList())
            }
        }
    }

    fun rebootDevice(device: Device) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf("reboot"), device)
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            } finally {
                refreshDevices()
            }
        }
    }
}