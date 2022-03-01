package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import models.emulator.Emulator
import models.emulator.EmulatorParameters
import util.ExecuteHelper
import util.MessageHelper
import util.PreferencesRepository

@ExperimentalCoroutinesApi
object EmulatorsService {
    private const val PATH_NAME = "emulator-parameters"

    private val repository = PreferencesRepository(PATH_NAME, EmulatorParameters::class.java)

    private val mutableEmulators: MutableStateFlow<List<Emulator>> = MutableStateFlow(emptyList())

    val emulators: StateFlow<List<Emulator>>
        get() = mutableEmulators

    init {
        repository.migrateMapV1ToV2("emulators") { oldKey ->
            oldKey.substring("emulator.".length, oldKey.length - ".parameters".length)
        }

        loadEmulators()
    }

    fun loadEmulators() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val emulators = ExecuteHelper.execute(SettingsService.emulatorPath, listOf("-list-avds"))
                    .split("\n")
                    .filter { line -> line.isNotEmpty() }
                    .map { name -> Emulator(name, repository.get(name) ?: EmulatorParameters()) }
                    .sortedBy { emulator -> emulator.name }
                mutableEmulators.value = emulators
            } catch (t: Throwable) {
                mutableEmulators.value = emptyList()
                MessageHelper.showThrowableMessage(t)
            }
        }
    }

    fun setEmulatorParameters(emulator: Emulator) {
        repository.set(emulator.name, emulator.parameters)
    }

    fun startEmulator(emulator: Emulator) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val arguments = mutableListOf("-avd", emulator.name)
                if (emulator.parameters.noSnapshotLoad) {
                    arguments.add("-no-snapshot-load")
                }
                if (emulator.parameters.writableSystem) {
                    arguments.add("-writable-system")
                }
                var bootCompleted = false
                ExecuteHelper.executeAsync(
                    command = SettingsService.emulatorPath,
                    arguments = arguments,
                    onData = { data ->
                        if (data.contains("boot completed")) {
                            bootCompleted = true
                            DevicesService.loadDevices()
                        }
                    },
                    onCompleted = {
                        if (!bootCompleted) {
                            DevicesService.loadDevices()
                            MessageHelper.showThrowableMessage(Throwable("Failed to start emulator"))
                        }
                    },
                    onError = { message ->
                        DevicesService.loadDevices()
                        if (message.isEmpty()) {
                            MessageHelper.showThrowableMessage(Throwable("Failed to start emulator"))
                        } else {
                            MessageHelper.showThrowableMessage(Throwable(message))
                        }
                    }
                )
            } catch (t: Throwable) {
                MessageHelper.showThrowableMessage(t)
                DevicesService.loadDevices()
            }
        }
    }
}