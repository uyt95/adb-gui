import androidx.compose.runtime.Composable
import androidx.compose.runtime.ExperimentalComposeApi
import components.Page
import kotlinx.coroutines.CoroutineScope
import models.Device
import pages.*

@ExperimentalComposeApi
object Router {
    val pages: List<Page> = listOf(ConnectionsPage(), EmulatorsPage(), KeyboardPage(), RemoteControlPage(), InstallPage(), SettingsPage())

    @Composable
    fun renderPage(route: String, mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        pages.find { page -> page.route == route }?.render(mainScope, devices, activeDevice)
    }
}