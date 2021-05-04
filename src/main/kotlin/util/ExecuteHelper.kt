package util

import models.Device
import services.SettingsService
import java.io.BufferedReader

object ExecuteHelper {
    class ExecuteError(message: String) : Throwable(message)

    fun execute(command: String, arguments: List<String>): String {
        val process = ProcessBuilder(listOf(command) + arguments)
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .start()

        process.waitFor()

        return if (process.exitValue() == 0) {
            process.inputStream.bufferedReader().use(BufferedReader::readText)
        } else {
            val errorText = process.errorStream.bufferedReader().use(BufferedReader::readText)
            throw ExecuteError(errorText)
        }
    }

    fun executeAdb(arguments: List<String>, device: Device? = null): String {
        return if (device == null) {
            execute(SettingsService.adbPath, arguments)
        } else {
            val args = mutableListOf("-s", device.address)
            args.addAll(arguments)
            execute(SettingsService.adbPath, args)
        }
    }
}