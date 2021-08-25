package pages

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.painter.BitmapPainter
import androidx.compose.ui.res.loadImageBitmap
import androidx.compose.ui.unit.dp
import components.Page
import components.vectorIconButton
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import services.DevicesService
import services.ScreenshotService
import services.SettingsService
import util.FileHelper
import java.io.File
import java.time.ZonedDateTime
import java.time.format.DateTimeFormatter

@ExperimentalCoroutinesApi
class ScreenshotPage : Page("screenshot", "Screenshot", scrollable = false) {
    companion object {
        const val extension = "png"
        val timestampFormatter: DateTimeFormatter by lazy {
            DateTimeFormatter.ofPattern("yyyyMMdd-hhmmss")
        }
    }

    @Composable
    override fun renderContent(mainScope: CoroutineScope) {
        val activeDevice by DevicesService.activeDevice.collectAsState()
        val directoryPath = remember { mutableStateOf(SettingsService.defaultScreenshotDirectory) }
        val bitmap: MutableState<ImageBitmap?> = remember { mutableStateOf(null) }

        Column(modifier = Modifier.fillMaxSize()) {
            Row(modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp), verticalAlignment = Alignment.CenterVertically) {
                TextField(value = directoryPath.value, onValueChange = { directoryPath.value = it }, modifier = Modifier.weight(1f))
                Button(modifier = Modifier.padding(start = 8.dp), onClick = {
                    FileHelper.saveDirectoryDialog {
                        directoryPath.value = it
                    }
                }) {
                    Text(text = "Browse")
                }
                vectorIconButton(name = "outline_screenshot_24dp", contentDescription = "take screenshot", enabled = activeDevice != null, modifier = Modifier.padding(start = 8.dp)) {
                    activeDevice?.let { device ->
                        val timestamp = ZonedDateTime.now().format(timestampFormatter)
                        val path = "${directoryPath.value}/screenshot-$timestamp.$extension"
                        ScreenshotService.takeScreenshot(device, path) { success ->
                            if (success) {
                                bitmap.value = File(path).inputStream().buffered().use(::loadImageBitmap)
                            }
                        }
                    }
                }
            }
            bitmap.value?.let {
                Image(painter = BitmapPainter(it), contentDescription = "screenshot preview", modifier = Modifier.fillMaxSize())
            }
        }
    }
}