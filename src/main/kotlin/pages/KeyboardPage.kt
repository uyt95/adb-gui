package pages

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.vectorXmlResource
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import components.Page
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import services.DevicesService
import services.KeyboardService

@ExperimentalCoroutinesApi
class KeyboardPage : Page("keyboard", "Keyboard") {
    @Composable
    override fun renderContent(mainScope: CoroutineScope) {
        val activeDevice by DevicesService.activeDevice.collectAsState()

        Column {
            val scope = rememberCoroutineScope()
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
                        KeyboardService.sendText(device, inputState.value.text) {
                            scope.launch {
                                inputState.value = TextFieldValue("")
                            }
                        }
                    }
                }) {
                    Image(imageVector = vectorXmlResource("icons/outline_send_24.xml"), contentDescription = "send")
                }
            }
        }
    }
}