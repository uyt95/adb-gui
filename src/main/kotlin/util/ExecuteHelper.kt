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
            var errorText = process.errorStream.bufferedReader().use(BufferedReader::readText)
            if (errorText.isEmpty()) {
                errorText = process.inputStream.bufferedReader().use(BufferedReader::readText)
            }
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

    fun executeAsync(command: String, arguments: List<String>, onData: (data: String) -> Unit, onCompleted: () -> Unit, onError: (message: String) -> Unit) {
        val process = ProcessBuilder(listOf(command) + arguments)
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .start()

        try {
            val reader = process.inputStream.bufferedReader()
            var line: String?
            do {
                line = reader.readLine()
                if (line != null) {
                    onData.invoke(line)
                }
            } while (line != null)

            val exitCode = process.waitFor()
            if (exitCode == 0) {
                onCompleted.invoke()
            } else {
                val errorText = process.errorStream.bufferedReader().use(BufferedReader::readText)
                onError.invoke(errorText)
            }
        } catch (t: Throwable) {
            onError.invoke(t.message ?: "Unknown execution error")
        }
    }
}