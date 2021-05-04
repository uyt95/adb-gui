import androidx.compose.desktop.AppManager
import androidx.compose.desktop.DesktopTheme
import androidx.compose.desktop.Window
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.vectorXmlResource
import androidx.compose.ui.unit.dp
import components.DeviceSelector
import kotlinx.coroutines.launch
import models.Device
import services.ConnectionsService
import services.DevicesService
import util.ErrorHelper
import java.awt.datatransfer.DataFlavor
import java.awt.dnd.DnDConstants
import java.awt.dnd.DropTarget
import java.awt.dnd.DropTargetDropEvent
import java.io.File

const val appTitle = "ADB GUI"

@ExperimentalComposeApi
@ExperimentalMaterialApi
fun main() = Window(title = appTitle) {
    val scope = rememberCoroutineScope()
    var activeRoute by remember { mutableStateOf(Router.pages.first().route) }

    var errorMessage: String? by remember { mutableStateOf(null) }
    ErrorHelper.mainErrorObserver = { value ->
        scope.launch {
            errorMessage = value
        }
    }

    var devices: List<Device> by remember { mutableStateOf(emptyList()) }
    var activeDevice: Device? by remember { mutableStateOf(null) }
    DevicesService.mainDevicesObserver = { value ->
        scope.launch {
            devices = value
            if (activeDevice == null || !value.contains(activeDevice)) {
                activeDevice = value.firstOrNull()
            }
        }
    }
    DevicesService.refreshDevices()

    var connectionsMap: Map<String, String> by remember { mutableStateOf(ConnectionsService.connections.associate { it.address to it.name }) }
    ConnectionsService.mainConnectionsObserver = { value ->
        scope.launch {
            connectionsMap = value.associate { it.address to it.name }
        }
    }

    MaterialTheme(Palette.lightColors) {
        DesktopTheme {
            Box(modifier = Modifier.fillMaxSize()) {
                Column(modifier = Modifier.fillMaxSize()) {
                    TopAppBar(title = {
                        Row(modifier = Modifier.height(50.dp).fillMaxWidth()) {
                            Text(appTitle, modifier = Modifier.align(Alignment.CenterVertically))
                            Box(modifier = Modifier.weight(1f)) {}
                            Box(modifier = Modifier.align(Alignment.CenterVertically).padding(end = 8.dp)) {
                                DeviceSelector.render(devices, activeDevice, connectionsMap) { activeDevice = it }
                            }
                            Button(modifier = Modifier.align(Alignment.CenterVertically), enabled = activeDevice != null, onClick = {
                                activeDevice?.let { device ->
                                    DevicesService.rebootDevice(device)
                                }
                            }) {
                                Image(imageVector = vectorXmlResource("icons/outline_power_settings_new_24.xml"), contentDescription = "reboot")
                            }
                        }
                    })
                    Row(modifier = Modifier.fillMaxWidth()) {
                        Box(modifier = Modifier.fillMaxHeight().width(200.dp)) {
                            val navScrollState = rememberScrollState(0)

                            Box(modifier = Modifier.fillMaxSize().verticalScroll(navScrollState)) {
                                Column {
                                    Router.pages.forEach { page ->
                                        navItem(page.title, page.route == activeRoute) {
                                            if (page.route != activeRoute) {
                                                activeRoute = page.route
                                            }
                                        }
                                    }
                                }
                            }
                            VerticalScrollbar(modifier = Modifier.align(Alignment.CenterEnd).fillMaxHeight(), adapter = rememberScrollbarAdapter(navScrollState))
                        }
                        Router.renderPage(activeRoute, scope, devices, activeDevice)
                    }
                }
                errorMessage?.let { message ->
                    Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.BottomCenter) {
                        Snackbar(modifier = Modifier.padding(8.dp)) {
                            Text(text = message)
                        }
                    }
                }
            }
        }
    }
}