package components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Button
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.vectorXmlResource
import models.Connection
import models.Device
import models.SelectOption
import services.DevicesService

@Composable
fun deviceSelector(devices: List<Device>, activeDevice: Device?, connections: List<Connection>, onDeviceSelected: (device: Device) -> Unit) {
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
            onSelected = { option -> onDeviceSelected.invoke(option.value) },
            modifier = Modifier.align(Alignment.CenterVertically)
        )
        Button({ DevicesService.refreshDevices() }, modifier = Modifier.align(Alignment.CenterVertically)) {
            Image(imageVector = vectorXmlResource("icons/outline_refresh_24.xml"), contentDescription = "refresh")
        }
    }
}
