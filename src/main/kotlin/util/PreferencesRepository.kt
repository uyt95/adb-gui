package util

import com.squareup.moshi.Types

class PreferencesRepository<T>(pathName: String, private val type: Class<T>) {
    private val preferences = PreferencesProvider.provide(pathName)
    private val jsonAdapter = JsonHelper.moshi.adapter(type)

    fun get(): List<T> {
        val itemCount = preferences.getInt(KEY_ITEM_COUNT, 0)
        val items = mutableListOf<T>()
        for (index in 0 until itemCount) {
            val json = preferences.get(getItemKey(index), null) ?: continue
            val item = jsonAdapter.fromJson(json) ?: continue
            items.add(item)
        }
        return items
    }

    fun get(key: String): T? {
        val json = preferences.get(key, null) ?: return null
        return jsonAdapter.fromJson(json)
    }

    fun set(items: List<T>) {
        preferences.putInt(KEY_ITEM_COUNT, items.size)
        items.forEachIndexed { index, item ->
            val json = jsonAdapter.toJson(item)
            preferences.put(getItemKey(index), json)
        }
    }

    fun set(key: String, item: T) {
        val json = jsonAdapter.toJson(item)
        preferences.put(key, json)
    }

    fun migrateListV1ToV2(pathName: String, key: String) {
        try {
            val preferencesV1 = PreferencesProvider.provideV1(pathName) ?: return
            val listAdapter = JsonHelper.moshi.adapter<List<T>>(
                Types.newParameterizedType(
                    List::class.java,
                    type
                )
            )
            val json = preferencesV1.get(key, "[]")
            listAdapter.fromJson(json)?.let {
                set(it)
            }
            preferencesV1.removeNode()
        } catch (_: Throwable) {
        }
    }

    fun migrateMapV1ToV2(pathName: String, convertKey: (oldKey: String) -> String) {
        try {
            val preferencesV1 = PreferencesProvider.provideV1(pathName) ?: return
            preferencesV1.keys().forEach { oldKey ->
                val json = preferencesV1.get(oldKey, null) ?: return@forEach
                val value = jsonAdapter.fromJson(json) ?: return@forEach
                val newKey = convertKey(oldKey)
                set(newKey, value)
            }
            preferencesV1.removeNode()
        } catch (_: Throwable) {
        }
    }

    companion object {
        private const val KEY_ITEM_COUNT = "itemCount"

        private fun getItemKey(index: Int): String {
            return "item[$index]"
        }
    }
}