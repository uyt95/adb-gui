package components

import Palette
import androidx.compose.desktop.DesktopTheme
import androidx.compose.desktop.Window
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

object Dialog {
    @Composable
    fun renderDialog(
        scope: CoroutineScope,
        show: MutableState<Boolean>,
        title: String,
        content: @Composable () -> Unit
    ) {
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

    @Composable
    fun renderDialogButtons(
        confirmText: String? = null,
        confirmEnabled: Boolean = true,
        onConfirm: (() -> Unit)? = null,
        cancelText: String? = null,
        onCancel: (() -> Unit)? = null,
    ) {
        Box(modifier = Modifier.fillMaxWidth(), contentAlignment = Alignment.CenterEnd) {
            Row {
                confirmText?.let {
                    Button(
                        modifier = Modifier.padding(end = 8.dp),
                        enabled = confirmEnabled,
                        onClick = { onConfirm?.invoke() }
                    ) {
                        Text(text = it)
                    }
                }
                cancelText?.let {
                    Button(onClick = { onCancel?.invoke() }) {
                        Text(text = it)
                    }
                }
            }
        }
    }
}