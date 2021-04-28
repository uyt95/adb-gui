package util

import com.squareup.moshi.Moshi
import com.squareup.moshi.kotlin.reflect.KotlinJsonAdapterFactory

object JsonHelper {
    val moshi: Moshi = Moshi.Builder().add(KotlinJsonAdapterFactory()).build()
}