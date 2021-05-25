package services

import com.squareup.moshi.Types
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import models.Connection
import util.MessageHelper
import util.ExecuteHelper
import util.JsonHelper
import java.util.prefs.Preferences

@ExperimentalCoroutinesApi
object ConnectionsService {
    private const val KEY_CONNECTIONS = "connections"

    private val preferences = Preferences.userRoot().node("connections")
    private val connectionListAdapter = JsonHelper.moshi.adapter<List<Connection>>(Types.newParameterizedType(List::class.java, Connection::class.java))

    private val mutableConnections: MutableStateFlow<List<Connection>> = MutableStateFlow(emptyList())

    val connections: StateFlow<List<Connection>>
        get() = mutableConnections

    init {
        val json = preferences.get(KEY_CONNECTIONS, "[]")
        mutableConnections.value = connectionListAdapter.fromJson(json)?.sortedBy { it.name.toLowerCase() } ?: emptyList()
    }

    fun setConnections(value: List<Connection>) {
        mutableConnections.value = value.sortedBy { it.name.toLowerCase() }
        val json = connectionListAdapter.toJson(value)
        preferences.put(KEY_CONNECTIONS, json)
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