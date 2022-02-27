package components

import Palette
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.MutableState
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.WindowPosition
import androidx.compose.ui.window.rememberDialogState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

@ExperimentalComposeUiApi
object Dialog {
    @Composable
    fun renderDialog(
        scope: CoroutineScope,
        show: MutableState<Boolean>,
        title: String,
        size: DpSize = DpSize(400.dp, Dp.Unspecified),
        content: @Composable () -> Unit
    ) {
        if (show.value) {
            val state = rememberDialogState(size = size, position = WindowPosition.PlatformDefault)

            Dialog(
                state = state,
                title = title,
                onCloseRequest = { scope.launch { show.value = false } }
            ) {
                MaterialTheme(Palette.lightColors) {
                    CompositionLocalProvider {
                        Box(modifier = Modifier.fillMaxWidth().padding(8.dp)) {
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