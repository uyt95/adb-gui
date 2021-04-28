package pages

import components.Page
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.CoroutineScope
import models.Device
import services.SettingsService

object SettingsPage : Page("settings", "Settings") {
    @Composable
    override fun renderContent(mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        Column {
            val adbPathState = remember { mutableStateOf(TextFieldValue(SettingsService.adbPath)) }
            val emulatorPathState = remember { mutableStateOf(TextFieldValue(SettingsService.emulatorPath)) }

            TextField(label = { Text("ADB path") }, modifier = Modifier.fillMaxWidth().padding(bottom=8.dp), value = adbPathState.value, onValueChange = {
                adbPathState.value = it
                SettingsService.adbPath = it.text
            })
            TextField(label = { Text("Emulator path") }, modifier = Modifier.fillMaxWidth(), value = emulatorPathState.value, onValueChange = {
                emulatorPathState.value = it
                SettingsService.emulatorPath = it.text
            })
        }
    }
}