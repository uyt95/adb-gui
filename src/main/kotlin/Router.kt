import androidx.compose.runtime.Composable
import components.Page
import kotlinx.coroutines.CoroutineScope
import models.Device
import pages.*

object Router {
    val pages: List<Page> = listOf(ConnectionsPage(), EmulatorsPage(), KeyboardPage(), RemoteControlPage(), SettingsPage())

    @Composable
    fun renderPage(route: String, mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        pages.find { page -> page.route == route }?.render(mainScope, devices, activeDevice)
    }
}