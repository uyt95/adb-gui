package pages

import androidx.compose.desktop.AppManager
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.*
import androidx.compose.ui.res.vectorXmlResource
import androidx.compose.ui.unit.dp
import components.Dialog
import components.Page
import components.table
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import models.Connection
import models.Device
import models.TableColumn
import services.ConnectionsService

class ConnectionsPage : Page("connections", "Connections", fab = Fab("+")) {
    @Composable
    override fun renderContent(mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        val scope = rememberCoroutineScope()
        val showAddDialog = remember { mutableStateOf(false) }
        val showEditDialog = remember { mutableStateOf(false) }
        val editDialogConnection: MutableState<Connection?> = remember { mutableStateOf(null) }
        var connections: List<Connection> by remember { mutableStateOf(emptyList()) }
        LaunchedEffect(connections) {
            connections = ConnectionsService.connections
        }

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
                                val newConnections = connections.toMutableList()
                                newConnections.remove(it)
                                ConnectionsService.connections = newConnections
                                connections = newConnections
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
                        ConnectionsService.connections = newConnections
                        connections = ConnectionsService.connections

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
                        ConnectionsService.connections = newConnections
                        connections = ConnectionsService.connections
                    }
                },
                onDismiss = {
                    scope.launch {
                        showEditDialog.value = false
                    }
                }
            )
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
                onValueChange = { address.value = it }
            )
            Dialog.renderDialogButtons(
                confirmText = if (connection == null) "Add" else "Save",
                confirmEnabled = name.value.isNotEmpty() && address.value.isNotEmpty(),
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