package models.emulator

data class Emulator(
    val name: String,
    val parameters: EmulatorParameters = EmulatorParameters()
)
