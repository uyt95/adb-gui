package components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Button
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.vectorXmlResource
import kotlinx.coroutines.ExperimentalCoroutinesApi
import models.SelectOption
import services.ConnectionsService
import services.DevicesService

@ExperimentalCoroutinesApi
@Composable
fun deviceSelector() {
    val devices by DevicesService.devices.collectAsState()
    val activeDevice by DevicesService.activeDevice.collectAsState()
    val connections by ConnectionsService.connections.collectAsState()

    val options = remember(devices, connections) {
        devices.map { device -> SelectOption(connections.find { connection -> connection.address == device.address }?.name ?: device.address, device) }
    }
    val selected = remember(options, activeDevice) {
        options.find { option -> option.value == activeDevice }
    }

    Row {
        select(
            options = options,
            selected = selected,
            onSelected = { option -> DevicesService.setActiveDevice(option.value) },
            modifier = Modifier.align(Alignment.CenterVertically)
        )
        Button({ DevicesService.loadDevices() }, modifier = Modifier.align(Alignment.CenterVertically)) {
            Image(imageVector = vectorXmlResource("icons/outline_refresh_24.xml"), contentDescription = "refresh")
        }
    }
}
