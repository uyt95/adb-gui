package components

import Palette
import androidx.compose.desktop.DesktopTheme
import androidx.compose.desktop.Window
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

@Composable
fun dialog(scope: CoroutineScope, show: MutableState<Boolean>, title: String, content: @Composable () -> Unit) {
    if (show.value) {
        Window(
            title = title,
            size = IntSize(400, 250),
            onDismissRequest = { scope.launch { show.value = false } },
        ) {
            MaterialTheme(Palette.lightColors) {
                DesktopTheme {
                    Box(modifier = Modifier.fillMaxSize().padding(8.dp)) {
                        content()
                    }
                }
            }
        }
    }
}