package pages

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Button
import androidx.compose.material.Checkbox
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.vectorXmlResource
import androidx.compose.ui.unit.dp
import components.Dialog
import components.Page
import components.table
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import models.emulator.Emulator
import models.TableColumn
import services.EmulatorsService

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
                        Button(modifier = Modifier.padding(end = 4.dp), onClick = {
                            scope.launch {
                                EmulatorsService.startEmulator(it)
                            }
                        }) { Image(imageVector = vectorXmlResource("icons/outline_play_arrow_24.xml"), contentDescription = "start") }
                        Button(onClick = {
                            scope.launch {
                                editDialogEmulator.value = it
                                showEditDialog.value = true
                            }
                        }) { Image(imageVector = vectorXmlResource("icons/outline_settings_24.xml"), contentDescription = "edit parameters") }
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
                            EmulatorsService.saveEmulatorParameters(emulator)

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