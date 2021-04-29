import androidx.compose.runtime.Composable
import components.Page
import kotlinx.coroutines.CoroutineScope
import models.Device
import pages.ConnectionsPage
import pages.EmulatorsPage
import pages.SettingsPage

object Router {
    val pages: List<Page> = listOf(ConnectionsPage(), EmulatorsPage(), SettingsPage())

    @Composable
    fun renderPage(route: String, mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        pages.find { page -> page.route == route }?.render(mainScope, devices, activeDevice)
    }
}