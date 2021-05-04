package util

import androidx.compose.desktop.AppManager
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import javax.swing.JFileChooser
import javax.swing.SwingUtilities
import javax.swing.UIManager

object FileHelper {
    private val isLinux: Boolean

    init {
        val os = System.getProperty("os.name")
        isLinux = os.contains("nix") || os.contains("nux")
    }

    fun openFileDialog(callback: (path: String) -> Unit) {
        CoroutineScope(Dispatchers.IO).launch {
            if (isLinux) {
                zenityOpenFileDialog(callback)
            } else {
                swingOpenFileDialog(callback)
            }
        }
    }

    private fun swingOpenFileDialog(callback: (path: String) -> Unit) {
        SwingUtilities.invokeLater {
            try {
                UIManager.setLookAndFeel(UIManager.getSystemLookAndFeelClassName())
            } catch (t: Throwable) {
            }
            try {
                val fileChooser = JFileChooser()
                val result = fileChooser.showOpenDialog(AppManager.focusedWindow?.window)
                if (result == JFileChooser.APPROVE_OPTION) {
                    callback.invoke(fileChooser.selectedFile.absolutePath.trim())
                }
            } catch (t: Throwable) {
            }
        }
    }

    private fun zenityOpenFileDialog(callback: (path: String) -> Unit) {
        try {
            var path = ExecuteHelper.execute("zenity", listOf("--file-selection", "--filename=${System.getProperty("user.home")}/"))
            path = path.trim()
            if (path.isNotEmpty()) {
                callback.invoke(path)
            }
        } catch (t: Throwable) {
            if (t !is ExecuteHelper.ExecuteError) {
                swingOpenFileDialog(callback)
            }
        }
    }
}