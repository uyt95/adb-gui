package models

data class RemoteControlButton(
    val icon: String?,
    val description: String,
    val key: String,
    val label: String? = null,
)
