package components

import androidx.compose.material.Button
import androidx.compose.material.DropdownMenu
import androidx.compose.material.DropdownMenuItem
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import models.SelectOption

@Composable
fun <T> select(options: List<SelectOption<T>>, selected: SelectOption<T>?, onSelected: (option: SelectOption<T>) -> Unit, enabled: Boolean = true, modifier: Modifier = Modifier) {
    var expanded by remember { mutableStateOf(false) }

    Button({ expanded = true }, modifier = modifier, enabled = enabled && options.isNotEmpty()) {
        Text(selected?.label ?: "--")
        DropdownMenu(
            expanded = expanded,
            onDismissRequest = { expanded = false },
            modifier = Modifier.align(Alignment.CenterVertically)
        ) {
            options.forEach { option ->
                DropdownMenuItem(onClick = {
                    expanded = false
                    onSelected.invoke(option)
                }) {
                    Text(text = option.label)
                }
            }
        }
    }
}