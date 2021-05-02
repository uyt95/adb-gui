package components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Button
import androidx.compose.material.DropdownMenu
import androidx.compose.material.DropdownMenuItem
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.vectorXmlResource
import models.Device
import services.DevicesService

object DeviceSelector {
    @Composable
    fun render(devices: List<Device>, activeDevice: Device?, connectionsMap: Map<String, String>, onDeviceSelected: (device: Device) -> Unit) {
        var expanded by remember { mutableStateOf(false) }

        Row {
            Button({ expanded = true }, modifier = Modifier.align(Alignment.CenterVertically), enabled = devices.isNotEmpty()) {
                Text(connectionsMap.getOrDefault(activeDevice?.address, activeDevice?.address) ?: "--")
                DropdownMenu(
                    expanded = expanded,
                    onDismissRequest = { expanded = false },
                    modifier = Modifier.align(Alignment.CenterVertically)
                ) {
                    devices.forEach { device ->
                        DropdownMenuItem(onClick = {
                            expanded = false
                            onDeviceSelected.invoke(device)
                        }) {
                            Text(text = connectionsMap.getOrDefault(device.address, device.address))
                        }
                    }
                }
            }
            Button({ DevicesService.refreshDevices() }, modifier = Modifier.align(Alignment.CenterVertically)) {
                Image(imageVector = vectorXmlResource("icons/outline_refresh_24.xml"), contentDescription = "refresh")
            }
        }
    }
}