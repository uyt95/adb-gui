package models

data class Emulator(
    val name: String,
    val parameters: EmulatorParameters = EmulatorParameters()
)
