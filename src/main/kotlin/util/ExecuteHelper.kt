package util

import models.Device
import services.SettingsService
import java.io.BufferedReader
import java.util.concurrent.TimeUnit

object ExecuteHelper {
    fun execute(command: String, arguments: List<String>): String {
        val process = ProcessBuilder(listOf(command) + arguments)
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .start()

        process.waitFor(60, TimeUnit.SECONDS)

        return if (process.exitValue() == 0) {
            process.inputStream.bufferedReader().use(BufferedReader::readText)
        } else {
            val errorText = process.errorStream.bufferedReader().use(BufferedReader::readText)
            throw Throwable(errorText)
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