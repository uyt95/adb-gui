package components

import androidx.compose.foundation.VerticalScrollbar
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.rememberScrollbarAdapter
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Divider
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.CoroutineScope
import models.Device

abstract class Page(val route: String, val title: String, val fab: Fab? = null) {
    data class Fab(val text: String, var onClick: (() -> Unit)? = null)

    @Composable
    protected abstract fun renderContent(mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?)

    @Composable
    fun render(mainScope: CoroutineScope, devices: List<Device>, activeDevice: Device?) {
        Column(modifier = Modifier.fillMaxSize()) {
            Column(modifier = Modifier.fillMaxWidth().padding(8.dp, 8.dp, 8.dp, 0.dp)) {
                Text(title, style = MaterialTheme.typography.h5)
                Divider(thickness = 2.dp)
            }
            Box(modifier = Modifier.fillMaxSize()) {
                val scrollState = rememberScrollState(0)

                Box(modifier = Modifier.fillMaxSize().verticalScroll(scrollState).padding(8.dp)) {
                    renderContent(mainScope, devices, activeDevice)
                }
                VerticalScrollbar(modifier = Modifier.align(Alignment.CenterEnd).fillMaxHeight(), adapter = rememberScrollbarAdapter(scrollState))
                if (fab != null) {
                    FloatingActionButton(onClick = { fab.onClick?.let { it() } }, modifier = Modifier.align(Alignment.BottomEnd).padding(16.dp)) {
                        Text(text = fab.text)
                    }
                }
            }
        }
    }
}