package models

data class EmulatorParameters(
    val noSnapshotLoad: Boolean = false,
    val writableSystem: Boolean = false
)
