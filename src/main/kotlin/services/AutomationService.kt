package services

import com.squareup.moshi.Types
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import models.Device
import models.automation.ArgumentType
import models.automation.Automation
import util.ExecuteHelper
import util.JsonHelper
import java.util.prefs.Preferences

object AutomationService {
    private const val KEY_AUTOMATIONS = "automations"

    private val preferences = Preferences.userRoot().node("automation")
    private val automationListAdapter = JsonHelper.moshi.adapter<List<Automation>>(Types.newParameterizedType(List::class.java, Automation::class.java))

    private val mutableAutomations: MutableStateFlow<List<Automation>> = MutableStateFlow<List<Automation>>(emptyList())

    val automations: StateFlow<List<Automation>>
        get() = mutableAutomations

    init {
        val json = preferences.get(KEY_AUTOMATIONS, "[]")
        mutableAutomations.value = automationListAdapter.fromJson(json)?.sortedBy { it.name.lowercase() } ?: emptyList()
    }

    fun setAutomations(value: List<Automation>) {
        mutableAutomations.value = value.sortedBy { it.name.lowercase() }
        val json = automationListAdapter.toJson(value)
        preferences.put(KEY_AUTOMATIONS, json)
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