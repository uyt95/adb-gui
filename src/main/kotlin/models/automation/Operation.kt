package models.automation

data class Operation(
    val key: String,
    val label: String,
    var arguments: List<Argument>
) {
    fun clone() = Operation(key, label, arguments.map { it.clone() })

    companion object {
        private val reboot: Operation
            get() = Operation("reboot", "Reboot", listOf(Argument.static("reboot")))
        private val keyCode: Operation
            get() = Operation("keyCode", "Send key code", listOf(Argument.static("shell"), Argument.static("input"), Argument.static("keyevent"), Argument.keyCode))
        private val keyboardInput: Operation
            get() = Operation("keyboardInput", "Send keyboard input", listOf(Argument.static("shell"), Argument.static("input"), Argument.static("keyboard"), Argument.static("text"), Argument.text))

        val adb = listOf(reboot, keyCode, keyboardInput)

        fun getOperation(key: String): Operation {
            return when (key) {
                "reboot" -> reboot
                "keyCode" -> keyCode
                "keyboardInput" -> keyboardInput
                else -> throw Throwable("Unknown operation key")
            }
        }
    }
}