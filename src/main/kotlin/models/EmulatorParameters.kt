package models

data class EmulatorParameters(
    var noSnapshotLoad: Boolean = false,
    var writableSystem: Boolean = false
)
