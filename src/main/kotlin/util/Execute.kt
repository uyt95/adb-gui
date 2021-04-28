package util

import java.io.BufferedReader
import java.util.concurrent.TimeUnit

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