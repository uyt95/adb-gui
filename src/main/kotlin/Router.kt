import androidx.compose.runtime.Composable
import androidx.compose.runtime.ExperimentalComposeApi
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.unit.ExperimentalUnitApi
import components.Page
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import pages.*

@ExperimentalUnitApi
@ExperimentalComposeUiApi
@ExperimentalCoroutinesApi
@ExperimentalComposeApi
object Router {
    val pages: List<Page> = listOf(ConnectionsPage(), EmulatorsPage(), KeyboardPage(), RemoteControlPage(), InstallPage(), AutomationsPage(), ProxiesPage(), ScreenshotPage(), SettingsPage())

    @Composable
    fun renderPage(route: String, mainScope: CoroutineScope) {
        pages.find { page -> page.route == route }?.render(mainScope)
    }
}