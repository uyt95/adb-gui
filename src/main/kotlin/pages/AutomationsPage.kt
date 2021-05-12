package pages

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.Card
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
import components.select
import components.table
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import models.TableColumn
import models.automation.*
import services.AutomationService
import services.DevicesService

@ExperimentalComposeApi
@ExperimentalCoroutinesApi
class AutomationsPage : Page("automations", "Automations", fab = Fab("+")) {
    @Composable
    override fun renderContent(mainScope: CoroutineScope) {
        val scope = rememberCoroutineScope()
        val automations by AutomationService.automations.collectAsState()
        val activeDevice by DevicesService.activeDevice.collectAsState()
        val showAddDialog = remember { mutableStateOf(false) }
        val showEditDialog = remember { mutableStateOf(false) }
        val editDialogAutomation: MutableState<Automation?> = remember { mutableStateOf(null) }
        val showConfirmDeleteDialog = remember { mutableStateOf(false) }
        val confirmDeleteDialogAutomation: MutableState<Automation?> = remember { mutableStateOf(null) }

        fab?.onClick = {
            scope.launch {
                showAddDialog.value = true
            }
        }

        table(
            listOf(
                TableColumn("Name") { Text(text = it.name) },
                TableColumn("Actions", contentAlignment = Alignment.Center) {
                    Row {
                        Button(modifier = Modifier.padding(end = 4.dp), onClick = {
                            scope.launch {
                                AutomationService.runAutomation(activeDevice, it)
                            }
                        }) { Image(imageVector = vectorXmlResource("icons/outline_play_arrow_24.xml"), contentDescription = "start") }
                        Button(modifier = Modifier.padding(end = 4.dp), onClick = {
                            scope.launch {
                                editDialogAutomation.value = it
                                showEditDialog.value = true
                            }
                        }) { Image(imageVector = vectorXmlResource("icons/outline_edit_24.xml"), contentDescription = "edit") }
                        Button(onClick = {
                            scope.launch {
                                confirmDeleteDialogAutomation.value = it
                                showConfirmDeleteDialog.value = true
                            }
                        }) { Image(imageVector = vectorXmlResource("icons/outline_delete_24.xml"), contentDescription = "remove") }
                    }
                }
            ),
            automations
        )

        Dialog.renderDialog(show = showAddDialog, scope = scope, title = "Add automation", content = {
            renderEditAutomationDialogContent(
                automation = null,
                onSave = { _, name, commands ->
                    scope.launch {
                        val newAutomations = automations.toMutableList()
                        newAutomations.add(Automation(name, commands))
                        AutomationService.setAutomations(newAutomations)

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

        Dialog.renderDialog(show = showEditDialog, scope = scope, title = "Edit automation", content = {
            renderEditAutomationDialogContent(
                automation = editDialogAutomation.value,
                onSave = { automation, name, commands ->
                    scope.launch {
                        val newAutomations = automations.toMutableList()
                        newAutomations.remove(automation)
                        newAutomations.add(Automation(name, commands))
                        AutomationService.setAutomations(newAutomations)
                    }
                },
                onDismiss = {
                    scope.launch {
                        showEditDialog.value = false
                    }
                }
            )
        })

        Dialog.renderDialog(show = showConfirmDeleteDialog, scope = scope, title = "Remove automation", content = {
            Column {
                Text(
                    text = "Are you sure you want to remove ${confirmDeleteDialogAutomation.value?.name}?",
                    modifier = Modifier.padding(bottom = 8.dp)
                )
                Dialog.renderDialogButtons(
                    "Remove",
                    onConfirm = {
                        val newAutomations = automations.toMutableList()
                        newAutomations.remove(confirmDeleteDialogAutomation.value)
                        AutomationService.setAutomations(newAutomations)

                        showConfirmDeleteDialog.value = false
                    },
                    cancelText = "Cancel",
                    onCancel = { showConfirmDeleteDialog.value = false }
                )
            }
        })
    }

    @Composable
    private fun renderEditAutomationDialogContent(
        automation: Automation?,
        onSave: (automation: Automation?, name: String, commands: List<Command>) -> Unit,
        onDismiss: () -> Unit
    ) {
        val name = remember { mutableStateOf(automation?.name ?: "") }
        val nameFocusRequester = remember { FocusRequester() }
        val commands = remember { mutableStateOf(automation?.commands?.map { it.clone() } ?: emptyList()) }

        DisposableEffect(true) {
            nameFocusRequester.requestFocus()
            onDispose { }
        }

        Column {
            TextField(
                label = { Text("Name") },
                modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp).focusOrder(nameFocusRequester),
                singleLine = true,
                value = name.value,
                onValueChange = { name.value = it }
            )
            commands.value.forEachIndexed { index, command ->
                commandField(
                    command = command,
                    canMoveUp = index > 0,
                    canMoveDown = index < commands.value.size - 1,
                    onChanged = {
                        val newCommands = commands.value.toMutableList()
                        if (it == null) {
                            newCommands.removeAt(index)
                        } else {
                            newCommands[index] = it
                        }
                        commands.value = newCommands
                    },
                    onMove = { direction ->
                        val newCommands = commands.value.toMutableList()
                        val tempCommand = newCommands[index]
                        newCommands[index] = newCommands[index + direction]
                        newCommands[index + direction] = tempCommand
                        commands.value = newCommands
                    }
                )
            }
            Box(modifier = Modifier.fillMaxWidth(), contentAlignment = Alignment.Center) {
                Button(onClick = {
                    val newCommands = commands.value.toMutableList()
                    newCommands.add(Command.getCommand(Automation.commandOptions.first().value))
                    commands.value = newCommands
                }) {
                    Text(text = "+")
                }
            }
            Dialog.renderDialogButtons(
                confirmText = if (automation == null) "Add" else "Save",
                confirmEnabled = name.value.trim().isNotEmpty() && commands.value.isNotEmpty(),
                onConfirm = {
                    onSave.invoke(automation, name.value, commands.value)
                    onDismiss.invoke()
                },
                cancelText = "Cancel",
                onCancel = { onDismiss.invoke() }
            )
        }
    }

    @Composable
    private fun commandField(command: Command, canMoveUp: Boolean, canMoveDown: Boolean, onChanged: (command: Command?) -> Unit, onMove: (direction: Int) -> Unit) {
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.fillMaxWidth()) {
                Row(modifier = Modifier.fillMaxWidth()) {
                    select(
                        options = Automation.commandOptions,
                        selected = Automation.commandOptions.find { it.value == command.key },
                        onSelected = { onChanged.invoke(Command.getCommand(it.value)) },
                        modifier = Modifier.weight(1f)
                    )
                    Button(enabled = canMoveUp, onClick = { onMove.invoke(-1) }) {
                        Image(imageVector = vectorXmlResource("icons/outline_north_24.xml"), contentDescription = "remove")
                    }
                    Button(enabled = canMoveDown, onClick = { onMove.invoke(1) }) {
                        Image(imageVector = vectorXmlResource("icons/outline_south_24.xml"), contentDescription = "remove")
                    }
                    Button(onClick = { onChanged.invoke(null) }) {
                        Image(imageVector = vectorXmlResource("icons/outline_delete_24.xml"), contentDescription = "remove")
                    }
                }
                operationField(command.key, command.operation) {
                    val newCommand = Command.getCommand(command.key)
                    newCommand.operation = it
                    onChanged.invoke(newCommand)
                }
            }
        }
    }

    @Composable
    private fun operationField(commandKey: String, operation: Operation, onChanged: (operation: Operation) -> Unit) {
        select(
            options = Command.getOperationOptions(commandKey),
            selected = Command.getOperationOptions(commandKey).find { it.value == operation.key },
            onSelected = { onChanged.invoke(Operation.getOperation(it.value)) }
        )
        operation.arguments.forEachIndexed { index, argument ->
            argumentField(argument) {
                val newOperation = Operation.getOperation(operation.key)
                val newArguments = newOperation.arguments.toMutableList()
                newArguments[index] = it
                newOperation.arguments = newArguments
                onChanged.invoke(newOperation)
            }
        }
    }

    @Composable
    private fun argumentField(argument: Argument, onChanged: (argument: Argument) -> Unit) {
        when (argument.type) {
            ArgumentType.Select -> select(
                options = Argument.getSelectOptions(argument),
                selected = Argument.getSelectOptions(argument).find { it.value == argument.value },
                onSelected = {
                    val newArgument = Argument.getArgument(argument.key)
                    newArgument.value = it.value
                    onChanged.invoke(newArgument)
                }
            )
            ArgumentType.Static -> {
            }
            ArgumentType.Text -> TextField(
                label = { Text("Name") },
                modifier = Modifier.fillMaxWidth(),
                singleLine = false,
                value = argument.value,
                onValueChange = {
                    val newArgument = Argument.getArgument(argument.key)
                    newArgument.value = it
                    onChanged.invoke(newArgument)
                }
            )
        }
    }
}