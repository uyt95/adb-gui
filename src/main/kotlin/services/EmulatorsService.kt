package services

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import models.Emulator
import models.EmulatorParameters
import util.ErrorHelper
import util.JsonHelper
import util.execute
import java.util.prefs.Preferences

object EmulatorsService {
    private const val KEY_EMULATOR = "emulator";
    private const val KEY_PARAMETERS = "parameters";

    private val preferences = Preferences.userRoot().node("emulators")
    private val emulatorParametersAdapter = JsonHelper.moshi.adapter(EmulatorParameters::class.java)

    fun getEmulators(callback: (emulators: List<Emulator>) -> Unit) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val emulators = execute(SettingsService.emulatorPath, listOf("-list-avds"))
                    .split("\n")
                    .filter { line -> line.isNotEmpty() }
                    .map { name -> Emulator(name, getEmulatorParameters(name)) }
                    .sortedBy { emulator -> emulator.name }
                callback.invoke(emulators)
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            }
        }
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
                val response = execute(SettingsService.emulatorPath, arguments)
                if (!response.contains("boot completed")) {
                    throw Throwable(response)
                }
            } catch (t: Throwable) {
                ErrorHelper.handleThrowable(t)
            } finally {
                DevicesService.refreshDevices()
            }
        }
    }

    fun saveEmulatorParameters(emulator: Emulator, newEmulatorParameters: EmulatorParameters) {
        val json = emulatorParametersAdapter.toJson(newEmulatorParameters)
        preferences.put("$KEY_EMULATOR.${emulator.name}.$KEY_PARAMETERS", json)
    }

    private fun getEmulatorParameters(name: String): EmulatorParameters {
        val json = preferences.get("$KEY_EMULATOR.$name.$KEY_PARAMETERS", "{}")
        return emulatorParametersAdapter.fromJson(json) ?: EmulatorParameters()
    }
}