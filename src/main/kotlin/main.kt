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
import components.deviceSelector
import components.scrollView
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import services.DevicesService
import util.ErrorHelper
import java.awt.image.BufferedImage
import javax.imageio.ImageIO

const val appTitle = "ADB GUI"

@ExperimentalCoroutinesApi
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

    val activeDevice by DevicesService.activeDevice.collectAsState()

    MaterialTheme(Palette.lightColors) {
        DesktopTheme {
            Box(modifier = Modifier.fillMaxSize()) {
                Column(modifier = Modifier.fillMaxSize()) {
                    TopAppBar(title = {
                        Row(modifier = Modifier.height(50.dp).fillMaxWidth()) {
                            Text(appTitle, modifier = Modifier.align(Alignment.CenterVertically))
                            Box(modifier = Modifier.weight(1f)) {}
                            Box(modifier = Modifier.align(Alignment.CenterVertically).padding(end = 8.dp)) {
                                deviceSelector()
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
                            scrollView {
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
                        }
                        Router.renderPage(activeRoute, scope)
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