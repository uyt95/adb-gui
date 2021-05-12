package models.automation

import models.KeyCode
import models.SelectOption

data class Argument(
    val key: String,
    val label: String,
    val type: ArgumentType,
    var value: String
) {
    fun clone() = Argument(key, label, type, value)

    companion object {
        private val keyCodeOptions: List<SelectOption<String>> = KeyCode.all.map { SelectOption(it.label, it.code) }.sortedBy { it.label }

        val keyCode: Argument
            get() = Argument("keyCode", "Key code", ArgumentType.Select, keyCodeOptions.first().value)
        fun static(value: String): Argument = Argument("static", "Static", ArgumentType.Static, value)
        val text: Argument
            get() = Argument("text", "Text", ArgumentType.Text, "")

        fun getArgument(key: String): Argument {
            return when (key) {
                "keyCode" -> keyCode
                "text" -> text
                else -> throw Throwable("Unknown argument key")
            }
        }

        fun getSelectOptions(argument: Argument): List<SelectOption<String>> {
            if (argument.type != ArgumentType.Select) {
                throw Throwable("Argument is not of type 'select'")
            }
            return when (argument.key) {
                keyCode.key -> keyCodeOptions
                else -> throw Throwable("Invalid select argument options key")
            }
        }
    }
}