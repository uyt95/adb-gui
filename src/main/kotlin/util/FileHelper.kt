package util

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

    fun saveDirectoryDialog(callback: (path: String) -> Unit) {
        CoroutineScope(Dispatchers.IO).launch {
            if (isLinux) {
                zenitySaveDirectoryDialog(callback)
            } else {
                swingSaveDirectoryDialog(callback)
            }
        }
    }

    private fun swingOpenFileDialog(callback: (path: String) -> Unit) {
        SwingUtilities.invokeLater {
            swingSetLookAndFeel()
            try {
                val fileChooser = JFileChooser()
                val result = fileChooser.showOpenDialog(WindowManager.appWindow)
                if (result == JFileChooser.APPROVE_OPTION) {
                    callback.invoke(fileChooser.selectedFile.absolutePath.trim())
                }
            } catch (t: Throwable) {
            }
        }
    }

    private fun swingSaveDirectoryDialog(callback: (path: String) -> Unit) {
        SwingUtilities.invokeLater {
            swingSetLookAndFeel()
            try {
                val fileChooser = JFileChooser()
                fileChooser.fileSelectionMode = JFileChooser.DIRECTORIES_ONLY
                val result = fileChooser.showSaveDialog(WindowManager.appWindow)
                if (result == JFileChooser.APPROVE_OPTION) {
                    callback.invoke(fileChooser.selectedFile.absolutePath.trim())
                }
            } catch (t: Throwable) {
            }
        }
    }

    private fun swingSetLookAndFeel() {
        try {
            UIManager.setLookAndFeel(UIManager.getSystemLookAndFeelClassName())
        } catch (t: Throwable) {
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

    private fun zenitySaveDirectoryDialog(callback: (path: String) -> Unit) {
        try {
            var path = ExecuteHelper.execute("zenity", listOf("--file-selection", "--save", "--directory", "--filename=${System.getProperty("user.home")}/"))
            path = path.trim()
            if (path.isNotEmpty()) {
                callback.invoke(path)
            }
        } catch (t: Throwable) {
            if (t !is ExecuteHelper.ExecuteError) {
                swingSaveDirectoryDialog(callback)
            }
        }
    }
}