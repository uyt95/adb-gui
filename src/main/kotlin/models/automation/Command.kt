package models.automation

import models.SelectOption

data class Command(
    val key: String,
    val label: String,
    val command: String,
    var operation: Operation
) {
    fun clone() = Command(key, label, command, operation.clone())

    companion object {
        val adb: Command
            get() = Command("adb", "ADB", "adb", Operation.getOperation(getOperationOptions("adb").first().value))

        fun getOperationOptions(key: String): List<SelectOption<String>> {
            return when (key) {
                "adb" -> Operation.adb.map { SelectOption(it.label, it.key) }
                else -> throw Throwable("Unknown operation options key")
            }
        }

        fun getCommand(key: String): Command {
            return when (key) {
                "adb" -> adb
                else -> throw Throwable("Unknown command key")
            }
        }
    }
}