package pages

import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.Card
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusOrder
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.WindowSize
import components.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import models.TableColumn
import models.automation.*
import services.AutomationService
import services.DevicesService

@ExperimentalComposeUiApi
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
                        vectorIconButton(name = "outline_play_arrow_24", contentDescription = "start", modifier = Modifier.padding(end = 4.dp)) {
                            scope.launch {
                                AutomationService.runAutomation(activeDevice, it)
                            }
                        }
                        vectorIconButton(name = "outline_edit_24", contentDescription = "edit", modifier = Modifier.padding(end = 4.dp)) {
                            scope.launch {
                                editDialogAutomation.value = it
                                showEditDialog.value = true
                            }
                        }
                        vectorIconButton(name = "outline_delete_24", contentDescription = "remove") {
                            scope.launch {
                                confirmDeleteDialogAutomation.value = it
                                showConfirmDeleteDialog.value = true
                            }
                        }
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

        Column(modifier = Modifier.padding(bottom = 8.dp)) {
            scrollView(modifier = Modifier.fillMaxWidth().weight(1f).padding(bottom = 8.dp)) {
                Column {
                    TextField(
                        label = { Text("Name") },
                        modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp).focusOrder(nameFocusRequester),
                        singleLine = true,
                        value = name.value,
                        onValueChange = { name.value = it }
                    )
                    commands.value.forEachIndexed { index, command ->
                        Card(modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp)) {
                            Column(modifier = Modifier.fillMaxWidth().padding(8.dp)) {
                                Row(modifier = Modifier.fillMaxWidth()) {
                                    Text(
                                        text = "Command #${index + 1}",
                                        modifier = Modifier.weight(1f).align(Alignment.CenterVertically).padding(end = 8.dp, bottom = 8.dp)
                                    )
                                    vectorIconButton(
                                        name = "outline_north_24",
                                        contentDescription = "move up",
                                        modifier = Modifier.padding(end = 4.dp),
                                        enabled = index > 0
                                    ) {
                                        val newCommands = commands.value.toMutableList()
                                        val tempCommand = newCommands[index]
                                        newCommands[index] = newCommands[index - 1]
                                        newCommands[index - 1] = tempCommand
                                        commands.value = newCommands
                                    }
                                    vectorIconButton(
                                        name = "outline_south_24",
                                        contentDescription = "move down",
                                        modifier = Modifier.padding(end = 4.dp),
                                        enabled = index < commands.value.size - 1
                                    ) {
                                        val newCommands = commands.value.toMutableList()
                                        val tempCommand = newCommands[index]
                                        newCommands[index] = newCommands[index + 1]
                                        newCommands[index + 1] = tempCommand
                                        commands.value = newCommands
                                    }
                                    vectorIconButton(name = "outline_delete_24", contentDescription = "remove") {
                                        val newCommands = commands.value.toMutableList()
                                        newCommands.removeAt(index)
                                        commands.value = newCommands
                                    }
                                }
                                commandField(
                                    command = command,
                                    onChanged = {
                                        val newCommands = commands.value.toMutableList()
                                        newCommands[index] = it
                                        commands.value = newCommands
                                    }
                                )
                            }
                        }
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
    private fun commandField(command: Command, onChanged: (command: Command) -> Unit) {
        select(
            options = Automation.commandOptions,
            selected = Automation.commandOptions.find { it.value == command.key },
            onSelected = { onChanged.invoke(Command.getCommand(it.value)) },
            modifier = Modifier.padding(bottom = 8.dp)
        )
        operationField(command.key, command.operation) {
            val newCommand = Command.getCommand(command.key)
            newCommand.operation = it
            onChanged.invoke(newCommand)
        }
    }

    @Composable
    private fun operationField(commandKey: String, operation: Operation, onChanged: (operation: Operation) -> Unit) {
        select(
            options = Command.getOperationOptions(commandKey),
            selected = Command.getOperationOptions(commandKey).find { it.value == operation.key },
            onSelected = { onChanged.invoke(Operation.getOperation(it.value)) },
            modifier = Modifier.padding(bottom = 8.dp)
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
                label = { Text(text = argument.label) },
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