package pages

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.vectorXmlResource
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import components.Page
import kotlinx.coroutines.CoroutineScope
import models.Device
import services.KeyboardService

class KeyboardPage : Page("keyboard", "Keyboard") {
    @Composable
    override fun renderContent(mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        Column {
            val inputState = remember { mutableStateOf(TextFieldValue("")) }

            Row {
                Column {
                    Button(modifier = Modifier.padding(bottom = 8.dp), enabled = activeDevice != null, onClick = {
                        activeDevice?.let { device ->
                            KeyboardService.sendBackspaceKey(device)
                        }
                    }) {
                        Image(imageVector = vectorXmlResource("icons/outline_backspace_24.xml"), contentDescription = "backspace")
                    }
                    Button(enabled = activeDevice != null, onClick = {
                        activeDevice?.let { device ->
                            KeyboardService.sendTabKey(device)
                        }
                    }) {
                        Image(imageVector = vectorXmlResource("icons/outline_keyboard_tab_24.xml"), contentDescription = "tab")
                    }
                }
                TextField(
                    label = { Text("Keyboard input") },
                    modifier = Modifier.fillMaxHeight().weight(1f).padding(start = 8.dp, end = 8.dp),
                    singleLine = false,
                    value = inputState.value,
                    onValueChange = { inputState.value = it }
                )
                Button(enabled = activeDevice != null && inputState.value.text.isNotEmpty(), onClick = {
                    activeDevice?.let { device ->
                        KeyboardService.sendText(device, inputState.value.text)
                    }
                }) {
                    Image(imageVector = vectorXmlResource("icons/outline_send_24.xml"), contentDescription = "send")
                }
            }
        }
    }
}