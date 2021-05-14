package pages

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.key.Key
import androidx.compose.ui.input.key.KeysSet
import androidx.compose.ui.input.key.plus
import androidx.compose.ui.input.key.shortcuts
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import components.Page
import components.vectorIconButton
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
        val scope = rememberCoroutineScope()
        val inputState = remember { mutableStateOf(TextFieldValue("")) }

        fun sendKeyboard() {
            activeDevice?.let { device ->
                if (inputState.value.text.isNotEmpty()) {
                    KeyboardService.sendText(device, inputState.value.text) {
                        scope.launch {
                            inputState.value = TextFieldValue("")
                        }
                    }
                }
            }
        }

        Column(modifier = Modifier.shortcuts {
            on(Key.CtrlLeft + Key.Enter) {
                sendKeyboard()
            }
        }) {
            Row {
                Column {
                    vectorIconButton(
                        name = "outline_backspace_24",
                        contentDescription = "backspace",
                        enabled = activeDevice != null,
                        modifier = Modifier.padding(bottom = 8.dp)
                    ) {
                        activeDevice?.let { device ->
                            KeyboardService.sendBackspaceKey(device)
                        }
                    }
                    vectorIconButton(
                        name = "outline_keyboard_tab_24",
                        contentDescription = "tab",
                        enabled = activeDevice != null
                    ) {
                        activeDevice?.let { device ->
                            KeyboardService.sendTabKey(device)
                        }
                    }
                }
                TextField(
                    label = { Text("Keyboard input") },
                    modifier = Modifier.fillMaxHeight().weight(1f).padding(start = 8.dp, end = 8.dp),
                    singleLine = false,
                    value = inputState.value,
                    onValueChange = { inputState.value = it }
                )
                vectorIconButton(
                    name = "outline_send_24",
                    contentDescription = "send",
                    enabled = activeDevice != null && inputState.value.text.isNotEmpty()
                ) { sendKeyboard() }
            }
        }
    }
}