package models.proxy

data class Proxy(
    val type: ProxyType,
    val from: Socket,
    val to: Socket
)
