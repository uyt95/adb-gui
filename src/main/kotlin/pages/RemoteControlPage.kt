package pages

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.Card
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import components.Page
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.ExperimentalCoroutinesApi
import models.RemoteControlButton
import services.DevicesService
import services.RemoteControlService

@ExperimentalCoroutinesApi
class RemoteControlPage : Page("remote-control", "Remote control") {
    private val buttonGrid: List<List<RemoteControlButton?>> = listOf(
        listOf(RemoteControlButton("outline_power_settings_new_24", "power", "POWER"), null, RemoteControlButton("outline_mic_24", "voice search", "SEARCH")),
        listOf(null, RemoteControlButton("outline_arrow_drop_up_24", "up", "DPAD_UP")),
        listOf(RemoteControlButton("outline_arrow_left_24", "left", "DPAD_LEFT"), RemoteControlButton(null, "ok", "DPAD_CENTER"), RemoteControlButton("outline_arrow_right_24", "right", "DPAD_RIGHT")),
        listOf(null, RemoteControlButton("outline_arrow_drop_down_24", "down", "DPAD_DOWN")),
        listOf(RemoteControlButton("outline_arrow_back_24", "back", "BACK"), RemoteControlButton("outline_home_24", "home", "HOME"), RemoteControlButton("outline_menu_24", "menu", "MENU")),
        listOf(RemoteControlButton("outline_fast_rewind_24", "rewind", "MEDIA_REWIND"), RemoteControlButton("outline_play_arrow_24", "play/pause", "MEDIA_PLAY_PAUSE"), RemoteControlButton("outline_fast_forward_24", "fast forward", "MEDIA_FAST_FORWARD")),
        listOf(RemoteControlButton("outline_volume_up_24", "volume up", "VOLUME_UP"), null, RemoteControlButton("outline_expand_less_24", "channel down", "PAGE_DOWN")),
        listOf(RemoteControlButton("outline_volume_down_24", "volume down", "VOLUME_DOWN"), RemoteControlButton("outline_volume_off_24", "mute", "MUTE"), RemoteControlButton("outline_expand_more_24", "channel up", "PAGE_UP"))
    )

    @Composable
    override fun renderContent(mainScope: CoroutineScope) {
        val activeDevice by DevicesService.activeDevice.collectAsState()

        Card {
            Column(modifier = Modifier.padding(4.dp)) {
                buttonGrid.forEach { row ->
                    Row {
                        row.forEach { button ->
                            if (button == null) {
                                Box(modifier = Modifier.width(48.dp).height(48.dp).padding(2.dp)) {}
                            } else {
                                Button(
                                    modifier = Modifier.width(48.dp).height(48.dp).padding(2.dp),
                                    contentPadding = PaddingValues(4.dp),
                                    enabled = activeDevice != null,
                                    onClick = {
                                        activeDevice?.let { device ->
                                            RemoteControlService.sendButtonPress(device, button)
                                        }
                                    }
                                ) {
                                    button.icon?.let { icon ->
                                        Image(painter = painterResource("icons/$icon.xml"), contentDescription = button.description)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}