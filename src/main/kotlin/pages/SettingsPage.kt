package pages

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import components.Page
import kotlinx.coroutines.CoroutineScope
import services.SettingsService

class SettingsPage : Page("settings", "Settings") {
    @Composable
    override fun renderContent(mainScope: CoroutineScope) {
        Column {
            val adbPathState = remember { mutableStateOf(TextFieldValue(SettingsService.adbPath)) }
            val emulatorPathState = remember { mutableStateOf(TextFieldValue(SettingsService.emulatorPath)) }
            val defaultScreenshotDirectory = remember { mutableStateOf(TextFieldValue(SettingsService.defaultScreenshotDirectory)) }

            TextField(
                label = { Text("ADB path") },
                modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp),
                singleLine = true,
                value = adbPathState.value,
                onValueChange = {
                    adbPathState.value = it
                    SettingsService.adbPath = it.text
                }
            )
            TextField(
                label = { Text("Emulator path") },
                modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp),
                singleLine = true,
                value = emulatorPathState.value,
                onValueChange = {
                    emulatorPathState.value = it
                    SettingsService.emulatorPath = it.text
                }
            )
            TextField(
                label = { Text("Default screenshot directory") },
                modifier = Modifier.fillMaxWidth(),
                singleLine = true,
                value = defaultScreenshotDirectory.value,
                onValueChange = {
                    defaultScreenshotDirectory.value = it
                    SettingsService.defaultScreenshotDirectory = it.text
                }
            )
        }
    }
}