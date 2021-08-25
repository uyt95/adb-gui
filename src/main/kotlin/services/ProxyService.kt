package services

import com.squareup.moshi.Types
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.launch
import models.Device
import models.proxy.Proxy
import models.proxy.ProxyType
import models.proxy.Socket
import util.MessageHelper
import util.ExecuteHelper
import util.JsonHelper
import java.util.prefs.Preferences

@ExperimentalCoroutinesApi
object ProxyService {
    private const val KEY_PROXIES = "proxies"

    private val preferences = Preferences.userRoot().node("proxies")
    private val proxyListAdapter = JsonHelper.moshi.adapter<List<Proxy>>(Types.newParameterizedType(List::class.java, Proxy::class.java))

    private val mutableSavedProxies: MutableStateFlow<List<Proxy>> = MutableStateFlow(emptyList())
    private val mutableAdbProxies: MutableStateFlow<List<Proxy>> = MutableStateFlow(emptyList())

    val savedProxies: StateFlow<List<Proxy>>
        get() = mutableSavedProxies

    val activeProxies: StateFlow<List<Proxy>>
        get() = mutableAdbProxies

    init {
        loadSavedProxies()

        CoroutineScope(Dispatchers.IO).launch {
            DevicesService.activeDevice.collect {
                loadActiveProxies()
            }
        }
    }

    fun setSavedProxies(value: List<Proxy>) {
        mutableSavedProxies.value = sortProxies(value)
        val json = proxyListAdapter.toJson(value)
        preferences.put(KEY_PROXIES, json)
    }

    fun removeSavedProxy(proxy: Proxy, device: Device?) {
        device?.let {
            disconnect(proxy, it)
        }

        val proxies = savedProxies.value.toMutableList()
        proxies.remove(proxy)
        setSavedProxies(proxies)
    }

    fun connect(proxy: Proxy, device: Device) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf(proxy.type.id, "${proxy.from.type.id}:${proxy.from.name}", "${proxy.to.type.id}:${proxy.to.name}"), device)
                if (response.trim() != proxy.from.name) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            } finally {
                loadActiveProxies()
            }
        }
    }

    fun disconnect(proxy: Proxy, device: Device) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val response = ExecuteHelper.executeAdb(listOf(proxy.type.id, "--remove", "${proxy.from.type.id}:${proxy.from.name}"), device)
                if (response.isNotEmpty()) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
            } finally {
                loadActiveProxies()
            }
        }
    }

    private fun loadSavedProxies() {
        val json = preferences.get(KEY_PROXIES, "[]")
        val parsedProxies = proxyListAdapter.fromJson(json) ?: emptyList()
        mutableSavedProxies.value = sortProxies(parsedProxies)
    }

    private fun sortProxies(proxies: List<Proxy>): List<Proxy> {
        return proxies.sortedWith(compareBy(
            { it.type.id.toLowerCase() },
            { it.from.type.id.toLowerCase() },
            { it.from.name.toLowerCase() },
            { it.to.type.id.toLowerCase() },
            { it.to.name.toLowerCase() }
        ))
    }

    private fun loadActiveProxies() {
        CoroutineScope(Dispatchers.IO).launch {
            var result: List<Proxy> = emptyList()
            DevicesService.activeDevice.value?.let { device ->
                try {
                    val forwardProxies = ExecuteHelper.executeAdb(listOf("forward", "--list"), device)
                        .split("\n")
                        .map { line -> line.split(Regex("\\s+")) }
                        .filter { tokens -> tokens.size == 3 }
                        .map { row ->
//                            val idParts = row[0].split(':')
//                            val address = if (idParts.size == 2) idParts[0] else row[0]
                            val from = Socket.forData(row[1])
                            val to = Socket.forData(row[2])
                            Proxy(ProxyType.Forward, from, to)
                        }

                    val reverseProxies = ExecuteHelper.executeAdb(listOf("reverse", "--list"), device)
                        .split("\n")
                        .map { line -> line.split(Regex("\\s+")) }
                        .filter { tokens -> tokens.size == 3 }
                        .map { row ->
//                            val idParts = row[0].split(':')
//                            val address = if (idParts.size == 2) idParts[0] else row[0]
                            val from = Socket.forData(row[1])
                            val to = Socket.forData(row[2])
                            Proxy(ProxyType.Reverse, from, to)
                        }

                    result = forwardProxies + reverseProxies
                } catch (t: Throwable) {
                    MessageHelper.showThrowableMessage(t)
                }
            }
            mutableAdbProxies.value = result
            setSavedProxies(savedProxies.value.union(result).toList())
        }
    }
}