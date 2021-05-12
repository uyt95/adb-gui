package models.automation

import models.SelectOption

data class Automation(
    val name: String,
    val commands: List<Command>
) {
    fun clone() = Automation(name, commands.map { it.clone() })

    companion object {
        val commandOptions: List<SelectOption<String>> = listOf(Command.adb).map { SelectOption(it.label, it.key) }
    }
}