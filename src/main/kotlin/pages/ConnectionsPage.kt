package pages

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusOrder
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.vectorXmlResource
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.TextUnitType
import androidx.compose.ui.unit.dp
import components.Dialog
import components.Page
import components.table
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import models.Connection
import models.TableColumn
import services.ConnectionsService
import services.DevicesService
import util.IpAddressHelper

@ExperimentalCoroutinesApi
@ExperimentalComposeApi
class ConnectionsPage : Page("connections", "Connections", fab = Fab("+")) {
    @Composable
    override fun renderContent(mainScope: CoroutineScope) {
        val scope = rememberCoroutineScope()
        val connections by ConnectionsService.connections.collectAsState()
        val devices by DevicesService.devices.collectAsState()
        val showAddDialog = remember { mutableStateOf(false) }
        val showEditDialog = remember { mutableStateOf(false) }
        val editDialogConnection: MutableState<Connection?> = remember { mutableStateOf(null) }
        val showConfirmDeleteDialog = remember { mutableStateOf(false) }
        val confirmDeleteDialogConnection: MutableState<Connection?> = remember { mutableStateOf(null) }

        fab?.onClick = {
            scope.launch {
                showAddDialog.value = true
            }
        }

        table(
            listOf(
                TableColumn("Name") { Text(text = it.name) },
                TableColumn("Address") { Text(text = it.address) },
                TableColumn("Status", contentAlignment = Alignment.Center) {
                    if (devices.any { device -> device.address == it.address }) {
                        Image(imageVector = vectorXmlResource("icons/outline_check_24.xml"), contentDescription = "connected")
                    } else {
                        Image(imageVector = vectorXmlResource("icons/outline_close_24.xml"), contentDescription = "disconnected")
                    }
                },
                TableColumn("Actions", contentAlignment = Alignment.Center) {
                    Row {
                        Button(modifier = Modifier.padding(end = 4.dp), onClick = {
                            scope.launch {
                                editDialogConnection.value = it
                                showEditDialog.value = true
                            }
                        }) { Image(imageVector = vectorXmlResource("icons/outline_edit_24.xml"), contentDescription = "edit") }
                        if (devices.any { device -> device.address == it.address }) {
                            Button(modifier = Modifier.padding(end = 4.dp), onClick = {
                                scope.launch {
                                    ConnectionsService.disconnect(it)
                                }
                            }) { Image(imageVector = vectorXmlResource("icons/outline_link_off_24.xml"), contentDescription = "disconnect") }
                        } else {
                            Button(modifier = Modifier.padding(end = 4.dp), onClick = {
                                scope.launch {
                                    ConnectionsService.connect(it)
                                }
                            }) { Image(imageVector = vectorXmlResource("icons/outline_link_24.xml"), contentDescription = "connect") }
                        }
                        Button(onClick = {
                            scope.launch {
                                confirmDeleteDialogConnection.value = it
                                showConfirmDeleteDialog.value = true
                            }
                        }) { Image(imageVector = vectorXmlResource("icons/outline_delete_24.xml"), contentDescription = "remove") }
                    }
                }
            ),
            connections
        )

        Dialog.renderDialog(show = showAddDialog, scope = scope, title = "Add connection", content = {
            renderEditConnectionDialogContent(
                connection = null,
                onSave = { _, name, address ->
                    scope.launch {
                        val newConnections = connections.toMutableList()
                        newConnections.add(Connection(name, address))
                        ConnectionsService.setConnections(newConnections)

                        showAddDialog.value = false
                    }
                },
                onDismiss = {
                    scope.launch {
                        showAddDialog.value = false
                    }
                }
            )
        })

        Dialog.renderDialog(show = showEditDialog, scope = scope, title = "Edit connection", content = {
            renderEditConnectionDialogContent(
                connection = editDialogConnection.value,
                onSave = { connection, name, address ->
                    scope.launch {
                        val newConnections = connections.toMutableList()
                        newConnections.remove(connection)
                        newConnections.add(Connection(name, address))
                        ConnectionsService.setConnections(newConnections)
                    }
                },
                onDismiss = {
                    scope.launch {
                        showEditDialog.value = false
                    }
                }
            )
        })

        Dialog.renderDialog(show = showConfirmDeleteDialog, scope = scope, title = "Remove connection", content = {
            Column {
                Text(
                    text = "Are you sure you want to remove ${confirmDeleteDialogConnection.value?.name}?",
                    modifier = Modifier.padding(bottom = 8.dp)
                )
                Dialog.renderDialogButtons(
                    "Remove",
                    onConfirm = {
                        val newConnections = connections.toMutableList()
                        newConnections.remove(confirmDeleteDialogConnection.value)
                        ConnectionsService.setConnections(newConnections)

                        showConfirmDeleteDialog.value = false
                    },
                    cancelText = "Cancel",
                    onCancel = { showConfirmDeleteDialog.value = false }
                )
            }
        })
    }

    @Composable
    private fun renderEditConnectionDialogContent(
        connection: Connection?,
        onSave: (connection: Connection?, name: String, address: String) -> Unit,
        onDismiss: () -> Unit
    ) {
        val name = remember { mutableStateOf(connection?.name ?: "") }
        val address = remember { mutableStateOf(connection?.address ?: "") }
        val addressValid = remember { mutableStateOf(false) }
        val nameFocusRequester = remember { FocusRequester() }
        val addressFocusRequester = remember { FocusRequester() }

        DisposableEffect(true) {
            nameFocusRequester.requestFocus()
            onDispose { }
        }

        Column {
            TextField(
                label = { Text("Name") },
                modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp).focusOrder(nameFocusRequester) { addressFocusRequester.requestFocus() },
                singleLine = true,
                value = name.value,
                onValueChange = { name.value = it }
            )
            TextField(
                label = { Text("Address") },
                modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp).focusOrder(addressFocusRequester),
                singleLine = true,
                value = address.value,
                onValueChange = {
                    address.value = it
                    addressValid.value = IpAddressHelper.validate(it)
                },
                isError = !addressValid.value
            )
            if (!addressValid.value) {
                Text(text = "Address has to be a valid IP-address", modifier = Modifier.padding(bottom = 8.dp), color = Color.Red, fontSize = TextUnit(10f, TextUnitType.Sp))
            }
            Dialog.renderDialogButtons(
                confirmText = if (connection == null) "Add" else "Save",
                confirmEnabled = name.value.trim().isNotEmpty() && address.value.trim().isNotEmpty() && addressValid.value,
                onConfirm = {
                    onSave.invoke(connection, name.value, address.value)
                    onDismiss.invoke()
                },
                cancelText = "Cancel",
                onCancel = { onDismiss.invoke() }
            )
        }
    }
}