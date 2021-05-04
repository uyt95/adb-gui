package pages

import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import components.Page
import kotlinx.coroutines.CoroutineScope
import models.Device
import services.InstallService
import util.FileHelper


class InstallPage : Page("install", "Install", canDndFiles = true) {
    @Composable
    override fun renderContent(mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        val installing = remember { mutableStateOf(false) }
        val filePath = remember { mutableStateOf("") }

        handleDndFiles = { paths ->
            paths.first()?.let { path ->
                if (path.isNotEmpty()) {
                    filePath.value = path
                }
            }
        }

        if (installing.value) {
            Box(modifier = Modifier.fillMaxWidth(), contentAlignment = Alignment.Center) {
                CircularProgressIndicator()
            }
        } else {
            Column {
                Row(modifier = Modifier.padding(bottom = 8.dp)) { }
                Row(modifier = Modifier.padding(bottom = 8.dp), verticalAlignment = Alignment.CenterVertically) {
                    TextField(
                        modifier = Modifier.weight(1f).padding(end = 8.dp),
                        label = { Text("Path") },
                        singleLine = true, value = filePath.value,
                        onValueChange = { filePath.value = it }
                    )
                    Button(onClick = {
                        FileHelper.openFileDialog {
                            filePath.value = it
                        }
                    }) {
                        Text(text = "Browse")
                    }
                }
                Row {
                    Button(enabled = filePath.value.isNotEmpty() && activeDevice != null, onClick = {
                        activeDevice?.let { device ->
                            installing.value = true
                            InstallService.install(device, filePath.value) {
                                installing.value = false
                            }
                        }
                    }) {
                        Text(text = "Install")
                    }
                }
            }
        }
    }
}