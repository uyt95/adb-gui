package pages

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import components.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import models.SelectOption
import models.TableColumn
import models.proxy.Proxy
import models.proxy.ProxyType
import models.proxy.Socket
import models.proxy.SocketType
import services.DevicesService
import services.ProxyService

@ExperimentalComposeUiApi
@ExperimentalCoroutinesApi
class ProxiesPage : Page("proxies", "Proxies", fab = Fab("+")) {
    @Composable
    override fun renderContent(mainScope: CoroutineScope) {
        val scope = rememberCoroutineScope()
        val savedProxies by ProxyService.savedProxies.collectAsState()
        val adbProxies by ProxyService.activeProxies.collectAsState()
        val activeDevice by DevicesService.activeDevice.collectAsState()
        val showAddDialog = remember { mutableStateOf(false) }
        val showEditDialog = remember { mutableStateOf(false) }
        val editDialogProxy: MutableState<Proxy?> = remember { mutableStateOf(null) }
        val showConfirmDeleteDialog = remember { mutableStateOf(false) }
        val confirmDeleteDialogProxy: MutableState<Proxy?> = remember { mutableStateOf(null) }

        fab?.onClick = {
            scope.launch {
                showAddDialog.value = true
            }
        }

        Text(text = "")
        table(
            listOf(
                TableColumn("Type") { Text(text = it.type.id) },
                TableColumn("From") { Text(text = "${it.from.type.id} ${it.from.name}") },
                TableColumn("To") { Text(text = "${it.to.type.id} ${it.to.name}") },
                TableColumn("Status", contentAlignment = Alignment.Center) { proxy ->
                    if (adbProxies.any { proxy == it }) {
                        Image(painter = painterResource("icons/outline_check_24.xml"), contentDescription = "connected")
                    } else {
                        Image(painter = painterResource("icons/outline_close_24.xml"), contentDescription = "disconnected")
                    }
                },
                TableColumn("Actions") { proxy ->
                    Row {
                        vectorIconButton(name = "outline_edit_24", contentDescription = "edit", modifier = Modifier.padding(end = 4.dp)) {
                            scope.launch {
                                editDialogProxy.value = proxy
                                showEditDialog.value = true
                            }
                        }
                        if (adbProxies.any { proxy == it }) {
                            vectorIconButton(name = "outline_link_off_24", contentDescription = "disconnect", enabled = activeDevice != null, modifier = Modifier.padding(end = 4.dp)) {
                                scope.launch {
                                    activeDevice?.let { device ->
                                        ProxyService.disconnect(proxy, device)
                                    }
                                }
                            }
                        } else {
                            vectorIconButton(name = "outline_link_24", contentDescription = "connect", enabled = activeDevice != null, modifier = Modifier.padding(end = 4.dp)) {
                                scope.launch {
                                    activeDevice?.let { device ->
                                        ProxyService.connect(proxy, device)
                                    }
                                }
                            }
                        }
                        vectorIconButton(name = "outline_delete_24", contentDescription = "remove") {
                            scope.launch {
                                confirmDeleteDialogProxy.value = proxy
                                showConfirmDeleteDialog.value = true
                            }
                        }
                    }
                }
            ),
            savedProxies
        )

        Dialog.renderDialog(show = showAddDialog, scope = scope, title = "Add proxy", content = {
            renderEditProxyDialogContent(
                proxy = null,
                onSave = { _, type, from, to ->
                    scope.launch {
                        val newProxies = savedProxies.toMutableList()
                        newProxies.add(Proxy(type, from, to))
                        ProxyService.setSavedProxies(newProxies)

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

        Dialog.renderDialog(show = showEditDialog, scope = scope, title = "Edit proxy", content = {
            renderEditProxyDialogContent(
                proxy = editDialogProxy.value,
                onSave = { proxy, type, from, to ->
                    scope.launch {
                        val newProxies = savedProxies.toMutableList()
                        newProxies.remove(proxy)
                        newProxies.add(Proxy(type, from, to))
                        ProxyService.setSavedProxies(newProxies)
                    }
                },
                onDismiss = {
                    scope.launch {
                        showEditDialog.value = false
                    }
                }
            )
        })

        Dialog.renderDialog(show = showConfirmDeleteDialog, scope = scope, title = "Remove proxy", content = {
            Column {
                Text(
                    text = "Are you sure you want to remove the proxy?",
                    modifier = Modifier.padding(bottom = 8.dp)
                )
                Dialog.renderDialogButtons(
                    "Remove",
                    onConfirm = {
                        confirmDeleteDialogProxy.value?.let { proxy ->
                            ProxyService.removeSavedProxy(proxy, activeDevice)
                        }
                        showConfirmDeleteDialog.value = false
                    },
                    cancelText = "Cancel",
                    onCancel = { showConfirmDeleteDialog.value = false }
                )
            }
        })
    }

    @Composable
    private fun renderEditProxyDialogContent(
        proxy: Proxy?,
        onSave: (proxy: Proxy?, type: ProxyType, from: Socket, to: Socket) -> Unit,
        onDismiss: () -> Unit
    ) {
        val proxyTypeOptions = remember { ProxyType.values().map { SelectOption(it.id, it) } }
        val proxyType = remember { mutableStateOf(proxyTypeOptions.find { proxy?.type == it.value } ?: proxyTypeOptions.first()) }
        val socketTypeOptions = remember(proxyType) { SocketType.values().filter { it.proxyTypes.contains(proxyType.value.value) }.map { SelectOption(it.id, it) } }
        val from = remember { mutableStateOf(proxy?.from ?: Socket(socketTypeOptions.first().value, "")) }
        val to = remember { mutableStateOf(proxy?.to ?: Socket(socketTypeOptions.first().value, "")) }

        Column {
            Row(modifier = Modifier.padding(bottom = 8.dp)) {
                Text(text = "Type:", modifier = Modifier.weight(0.25f))
                select(
                    options = proxyTypeOptions,
                    selected = proxyType.value,
                    onSelected = { proxyType.value = it },
                    modifier = Modifier.weight(0.75f)
                )
            }
            Row(modifier = Modifier.padding(bottom = 8.dp)) {
                Text(text = "From:", modifier = Modifier.weight(0.25f))
                renderEditSocketFields(
                    socketTypeOptions = socketTypeOptions,
                    socket = from,
                    modifier = Modifier.weight(0.75f)
                )
            }
            Row(modifier = Modifier.padding(bottom = 8.dp)) {
                Text(text = "To:", modifier = Modifier.weight(0.25f))
                renderEditSocketFields(
                    socketTypeOptions = socketTypeOptions,
                    socket = to,
                    modifier = Modifier.weight(0.75f)
                )
            }
            Dialog.renderDialogButtons(
                confirmText = if (proxy == null) "Add" else "Save",
                confirmEnabled = from.value.name.trim().isNotEmpty() && to.value.name.trim().isNotEmpty(),
                onConfirm = {
                    onSave.invoke(proxy, proxyType.value.value, from.value, to.value)
                    onDismiss.invoke()
                },
                cancelText = "Cancel",
                onCancel = { onDismiss.invoke() }
            )
        }
    }

    @Composable
    private fun renderEditSocketFields(socketTypeOptions: List<SelectOption<SocketType>>, socket: MutableState<Socket>, modifier: Modifier) {
        val type = remember { mutableStateOf(socketTypeOptions.find { socket.value.type == it.value } ?: socketTypeOptions.first()) }

        Row(modifier = modifier) {
            select(
                options = socketTypeOptions,
                selected = type.value,
                onSelected = {
                    type.value = it
                    socket.value = Socket(it.value, socket.value.name)
                },
                modifier = Modifier.padding(end = 8.dp)
            )
            TextField(
                label = { Text("Name") },
                modifier = Modifier.fillMaxWidth(),
                singleLine = true,
                value = socket.value.name,
                onValueChange = { socket.value = Socket(type.value.value, it) }
            )
        }
    }
}