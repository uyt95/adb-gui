package pages

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Checkbox
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import components.Dialog
import components.Page
import components.table
import components.vectorIconButton
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import models.TableColumn
import models.emulator.Emulator
import services.EmulatorsService

@ExperimentalComposeUiApi
@ExperimentalCoroutinesApi
class EmulatorsPage : Page("emulators", "Emulators") {
    @Composable
    override fun renderContent(mainScope: CoroutineScope) {
        val scope = rememberCoroutineScope()
        val showEditDialog = remember { mutableStateOf(false) }
        val editDialogEmulator: MutableState<Emulator?> = remember { mutableStateOf(null) }
        val emulators by EmulatorsService.emulators.collectAsState()

        LaunchedEffect(true) {
            EmulatorsService.loadEmulators()
        }

        table(
            listOf(
                TableColumn("Name") { Text(text = it.name) },
                TableColumn("Actions", contentAlignment = Alignment.Center) {
                    Row {
                        vectorIconButton(name = "outline_play_arrow_24", contentDescription = "start", modifier = Modifier.padding(end = 4.dp)) {
                            scope.launch {
                                EmulatorsService.startEmulator(it)
                            }
                        }
                        vectorIconButton(name = "outline_settings_24", contentDescription = "edit parameters") {
                            scope.launch {
                                editDialogEmulator.value = it
                                showEditDialog.value = true
                            }
                        }
                    }
                }
            ),
            emulators
        )

        Dialog.renderDialog(show = showEditDialog, scope = scope, title = "Edit parameters", content = {
            val emulator = remember { editDialogEmulator.value!! }
            val noSnapshotLoad = remember { mutableStateOf(emulator.parameters.noSnapshotLoad) }
            val writableSystem = remember { mutableStateOf(emulator.parameters.writableSystem) }

            Column {
                Row(modifier = Modifier.padding(bottom = 8.dp), verticalAlignment = Alignment.CenterVertically) {
                    Checkbox(checked = noSnapshotLoad.value, onCheckedChange = { noSnapshotLoad.value = it })
                    Text(text = "No snapshot load", modifier = Modifier.padding(start = 4.dp))
                }
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Checkbox(checked = writableSystem.value, onCheckedChange = { writableSystem.value = it })
                    Text(text = "Writable system", modifier = Modifier.padding(start = 4.dp))
                }
                Dialog.renderDialogButtons(
                    confirmText = "Save",
                    onConfirm = {
                        scope.launch {
                            emulator.parameters.noSnapshotLoad = noSnapshotLoad.value
                            emulator.parameters.writableSystem = writableSystem.value
                            EmulatorsService.setEmulatorParameters(emulator)

                            showEditDialog.value = false
                        }
                    },
                    cancelText = "Cancel",
                    onCancel = {
                        scope.launch {
                            showEditDialog.value = false
                        }
                    }
                )
            }
        })
    }
}