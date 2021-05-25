package util

import kotlinx.coroutines.*
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

object MessageHelper {
    private var hideJob: Job? = null
    private val mutex: Mutex = Mutex()

    var mainErrorObserver: ((error: String?) -> Unit)? = null

    fun showThrowableMessage(t: Throwable) {
        t.message?.let { message ->
            showMessage(message)
        }
    }

    fun showMessage(message: String) {
        CoroutineScope(Dispatchers.Default).launch {
            mutex.withLock {
                mainErrorObserver?.invoke(message)
                hideJob?.cancel()
                hideJob = CoroutineScope(Dispatchers.Default).launch {
                    mutex.withLock {
                        delay(3000)
                        mainErrorObserver?.invoke(null)
                    }
                }
            }
        }
    }
}