package components

import WindowManager
import androidx.compose.foundation.layout.*
import androidx.compose.material.Divider
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.CoroutineScope
import java.awt.datatransfer.DataFlavor
import java.awt.dnd.DnDConstants
import java.awt.dnd.DropTarget
import java.awt.dnd.DropTargetDropEvent
import java.io.File

abstract class Page(
    val route: String,
    val title: String,
    val fab: Fab? = null,
    private val canDndFiles:
    Boolean = false,
    private val scrollable: Boolean = true
) {
    data class Fab(val text: String, var onClick: (() -> Unit)? = null)

    protected var handleDndFiles: ((paths: List<String>) -> Unit)? = null

    @Composable
    protected abstract fun renderContent(mainScope: CoroutineScope)

    @Composable
    protected open fun fileDragAndDrop() {
        WindowManager.appWindow?.contentPane?.dropTarget = object : DropTarget() {
            override fun isActive(): Boolean {
                return canDndFiles
            }

            override fun drop(event: DropTargetDropEvent) {
                try {
                    event.acceptDrop(DnDConstants.ACTION_REFERENCE)
                    val droppedFiles = event.transferable.getTransferData(DataFlavor.javaFileListFlavor) as List<*>
                    val droppedPaths = droppedFiles.filterIsInstance<File>().map { it.absolutePath.trim() }.filter { it.isNotEmpty() }
                    handleDndFiles?.invoke(droppedPaths)
                } catch (t: Throwable) {
                }
            }
        }
    }

    @Composable
    fun render(mainScope: CoroutineScope) {
        Column(modifier = Modifier.fillMaxSize()) {
            Column(modifier = Modifier.fillMaxWidth().padding(8.dp, 8.dp, 8.dp, 0.dp)) {
                Text(title, style = MaterialTheme.typography.h5)
                Divider(thickness = 2.dp)
            }

            Box(modifier = Modifier.fillMaxSize()) {
                if (scrollable) {
                    scrollView {
                        renderContent(mainScope)
                    }
                } else {
                    Box(modifier = Modifier.fillMaxSize().padding(8.dp)) {
                        renderContent(mainScope)
                    }
                }
                if (fab != null) {
                    FloatingActionButton(onClick = { fab.onClick?.let { it() } }, modifier = Modifier.align(Alignment.BottomEnd).padding(16.dp)) {
                        Text(text = fab.text)
                    }
                }
            }
        }
        fileDragAndDrop()
    }
}