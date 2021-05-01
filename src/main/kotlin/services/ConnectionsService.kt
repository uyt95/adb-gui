package services

import com.squareup.moshi.Types
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import models.Connection
import util.ErrorHelper
import util.ExecuteHelper
import util.JsonHelper
import java.util.prefs.Preferences

object ConnectionsService {
    private const val KEY_CONNECTIONS = "connections"

    private val preferences = Preferences.userRoot().node("connections")
    private val connectionListAdapter = JsonHelper.moshi.adapter<List<Connection>>(Types.newParameterizedType(List::class.java, Connection::class.java))

    var mainConnectionsObserver: ((connections: List<Connection>) -> Unit)? = null

    var connections: List<Connection>
        get() {
            val json = preferences.get(KEY_CONNECTIONS, "[]")
            return connectionListAdapter.fromJson(json)?.sortedBy { it.name.toLowerCase() } ?: emptyList()
        }
        set(value) {
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
                ErrorHelper.handleThrowable(t)
            } finally {
                DevicesService.refreshDevices()
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
                ErrorHelper.handleThrowable(t)
            } finally {
                DevicesService.refreshDevices()
            }
        }
    }
}