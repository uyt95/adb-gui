package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import models.Device
import models.automation.ArgumentType
import models.automation.Automation
import util.ExecuteHelper
import util.PreferencesRepository

object AutomationService {
    private const val PATH_NAME = "automations"

    private val repository = PreferencesRepository(PATH_NAME, Automation::class.java)

    private val mutableAutomations: MutableStateFlow<List<Automation>> = MutableStateFlow<List<Automation>>(emptyList())

    val automations: StateFlow<List<Automation>>
        get() = mutableAutomations

    init {
        repository.migrateListV1ToV2("automation", "automations")

        mutableAutomations.value = repository.get().sortedBy { it.name.lowercase() }
    }

    fun setAutomations(value: List<Automation>) {
        val items = value.sortedBy { it.name.lowercase() }
        mutableAutomations.value = items
        repository.set(items)
    }

    fun runAutomation(device: Device?, automation: Automation) {
        CoroutineScope(Dispatchers.IO).launch {
            runAutomationHelper(device, automation, 0)
        }
    }

    private fun runAutomationHelper(device: Device?, automation: Automation, commandIndex: Int) {
        val command = automation.commands[commandIndex]
        val arguments = command.operation.arguments.map {
            if (it.type == ArgumentType.Text) {
                "'${it.value}'"
            } else {
                it.value
            }
        }
        val response = if (command.command == "adb") {
            ExecuteHelper.executeAdb(arguments, device)
        } else {
            ExecuteHelper.execute(command.command, arguments)
        }
        if (commandIndex + 1 < automation.commands.size) {
            runAutomationHelper(device, automation, commandIndex + 1)
        }
    }
}