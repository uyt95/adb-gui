package models.proxy

enum class SocketType(val id: String, val proxyTypes: List<ProxyType>) {
    Tcp("tcp", listOf(ProxyType.Forward, ProxyType.Reverse)),
    LocalAbstract("localabstract", listOf(ProxyType.Forward, ProxyType.Reverse)),
    LocalReserved("localreserved", listOf(ProxyType.Forward, ProxyType.Reverse)),
    LocalFileSystem("localfilesystem", listOf(ProxyType.Forward, ProxyType.Reverse)),
    Dev("dev", listOf(ProxyType.Forward)),
    JDWP("jdwp", listOf(ProxyType.Forward)),
    VSock("vsock", listOf(ProxyType.Forward)),
    AcceptFD("acceptfd", listOf(ProxyType.Forward));

    companion object {
        fun forId(id: String): SocketType {
            return values().find { it.id == id } ?: throw Throwable("Invalid socket type")
        }
    }
}