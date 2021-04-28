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
import components.Page
import components.dialog
import components.table
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import models.Connection
import models.Device
import models.TableColumn
import services.ConnectionsService

object ConnectionsPage : Page("connections", "Connections", fab = Fab("+")) {
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

        dialog(show = showAddDialog, scope = scope, title = "Add connection", content = {
            renderEditConnectionDialogContent(
                scope = scope,
                connection = null,
                onSave = { _, name, address ->
                    scope.launch {
                        val newConnections = connections.toMutableList()
                        newConnections.add(Connection(name, address))
                        ConnectionsService.connections = newConnections
                        connections = ConnectionsService.connections

                        showAddDialog.value = false
                        AppManager.focusedWindow?.close()
                    }
                },
                onDismiss = {
                    scope.launch {
                        showAddDialog.value = false
                        AppManager.focusedWindow?.close()
                    }
                }
            )
        })

        dialog(show = showEditDialog, scope = scope, title = "Edit connection", content = {
            renderEditConnectionDialogContent(
                scope = scope,
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
                        AppManager.focusedWindow?.close()
                    }
                }
            )
        })
    }

    @Composable
    private fun renderEditConnectionDialogContent(
        scope: CoroutineScope,
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
            Box(modifier = Modifier.fillMaxWidth(), contentAlignment = Alignment.CenterEnd) {
                Row {
                    Button(modifier = Modifier.padding(end = 8.dp), enabled = name.value.isNotEmpty() && address.value.isNotEmpty(), onClick = {
                        onSave.invoke(connection, name.value, address.value)
                        onDismiss.invoke()
                    }) {
                        if (connection == null) {
                            Text(text = "Add")
                        } else {
                            Text(text = "Save")
                        }
                    }
                    Button(onClick = {
                        scope.launch {
                            onDismiss.invoke()
                        }
                    }) {
                        Text(text = "Cancel")
                    }
                }
            }
        }
    }
}