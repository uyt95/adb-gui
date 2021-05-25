package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import models.Device
import util.MessageHelper
import util.ExecuteHelper

@ExperimentalCoroutinesApi
object DevicesService {
    private val mutableDevices: MutableStateFlow<List<Device>> = MutableStateFlow(emptyList())
    private val mutableActiveDevice: MutableStateFlow<Device?> = MutableStateFlow(null)

    val devices: StateFlow<List<Device>>
        get() = mutableDevices

    val activeDevice: StateFlow<Device?>
        get() = mutableActiveDevice

    init {
        loadDevices()
    }

    fun loadDevices() {
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
                mutableDevices.value = devices

                if (mutableActiveDevice.value == null || !devices.contains(mutableActiveDevice.value)) {
                    mutableActiveDevice.value = devices.firstOrNull()
                }
            } catch (t: Throwable) {
                mutableDevices.value = emptyList()
                mutableActiveDevice.value = null

                MessageHelper.showThrowableMessage(t)
            }
        }
    }

    fun setActiveDevice(device: Device) {
        mutableActiveDevice.value = device
    }

    fun rebootDevice(device: Device) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf("reboot"), device)
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            } finally {
                loadDevices()
            }
        }
    }
}