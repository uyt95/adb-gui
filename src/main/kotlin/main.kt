import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.ExperimentalUnitApi
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import components.deviceSelector
import components.scrollView
import components.vectorIconButton
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import services.DevicesService
import util.Constants
import util.MessageHelper

@ExperimentalUnitApi
@ExperimentalComposeUiApi
@ExperimentalCoroutinesApi
@ExperimentalComposeApi
@ExperimentalMaterialApi
fun main() = application {
    Window(onCloseRequest = ::exitApplication, title = Constants.appTitle, icon = painterResource("icon.png")) {
        WindowManager.appWindow = this.window

        val scope = rememberCoroutineScope()
        var activeRoute by remember { mutableStateOf(Router.pages.first().route) }

        var errorMessage: String? by remember { mutableStateOf(null) }
        MessageHelper.mainErrorObserver = { value ->
            scope.launch {
                errorMessage = value
            }
        }

        val activeDevice by DevicesService.activeDevice.collectAsState()

        MaterialTheme(Palette.lightColors) {
            CompositionLocalProvider {
                Box(modifier = Modifier.fillMaxSize()) {
                    Column(modifier = Modifier.fillMaxSize()) {
                        TopAppBar(title = {
                            Row(modifier = Modifier.height(50.dp).fillMaxWidth()) {
                                Text(Constants.appTitle, modifier = Modifier.align(Alignment.CenterVertically))
                                Box(modifier = Modifier.weight(1f)) {}
                                Box(modifier = Modifier.align(Alignment.CenterVertically).padding(end = 8.dp)) {
                                    deviceSelector()
                                }
                                vectorIconButton(
                                    name = "outline_power_settings_new_24",
                                    contentDescription = "reboot",
                                    enabled = activeDevice != null,
                                    modifier = Modifier.align(Alignment.CenterVertically)
                                ) {
                                    activeDevice?.let { device ->
                                        DevicesService.rebootDevice(device)
                                    }
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
}
