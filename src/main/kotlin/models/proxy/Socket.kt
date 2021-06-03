package models.proxy

data class Socket(
    val type: SocketType,
    val name: String
) {
    companion object {
        fun forData(data: String): Socket {
            val tokens = data.split(":")
            return Socket(SocketType.forId(tokens[0]), tokens.drop(1).joinToString(" "))
        }
    }
}
