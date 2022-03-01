package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import models.Connection
import util.ExecuteHelper
import util.MessageHelper
import util.PreferencesRepository

@ExperimentalCoroutinesApi
object ConnectionsService {
    private const val PATH_NAME = "connections"

    private val repository = PreferencesRepository(PATH_NAME, Connection::class.java)

    private val mutableConnections: MutableStateFlow<List<Connection>> = MutableStateFlow(emptyList())

    val connections: StateFlow<List<Connection>>
        get() = mutableConnections

    init {
        repository.migrateListV1ToV2("connections", "connections")

        mutableConnections.value = repository.get().sortedBy { it.name.lowercase() }
    }

    fun setConnections(value: List<Connection>) {
        val items = value.sortedBy { it.name.lowercase() }
        mutableConnections.value = items
        repository.set(items)
    }

    fun connect(connection: Connection) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val result = ExecuteHelper.executeAdb(listOf("connect", connection.address))
                if (!Regex("(already )?connected to .*", RegexOption.MULTILINE).containsMatchIn(result)) {
                    throw Throwable(result)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            } finally {
                DevicesService.loadDevices()
            }
        }
    }

    fun disconnect(connection: Connection) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val result = ExecuteHelper.executeAdb(listOf("disconnect", connection.address))
                if (
                    !Regex("disconnected .*", RegexOption.MULTILINE).containsMatchIn(result) &&
                    !Regex("error: no such device '.*'", RegexOption.MULTILINE).containsMatchIn(result)
                ) {
                    throw Throwable(result)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            } finally {
                DevicesService.loadDevices()
            }
        }
    }
}