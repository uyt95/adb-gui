import androidx.compose.runtime.Composable
import components.Page
import kotlinx.coroutines.CoroutineScope
import models.Device
import pages.ConnectionsPage
import pages.SettingsPage

object Router {
    val pages: List<Page> = listOf(ConnectionsPage, SettingsPage)

    @Composable
    fun renderPage(route: String, mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        pages.find { page -> page.route == route }?.render(mainScope, devices, activeDevice)
    }
}