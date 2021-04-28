package util

import kotlinx.coroutines.*
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

object ErrorHelper {
    private var hideJob: Job? = null
    private val mutex: Mutex = Mutex()

    var mainErrorObserver: ((error: String?) -> Unit)? = null

    fun handleThrowable(t: Throwable) {
        CoroutineScope(Dispatchers.Default).launch {
            mutex.withLock {
                t.message?.let { message ->
                    mainErrorObserver?.invoke(message)
                }
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