package components

import androidx.compose.foundation.layout.*
import androidx.compose.material.Card
import androidx.compose.material.Divider
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import models.TableColumn

@Composable
fun <T> table(columns: List<TableColumn<T>>, data: List<T>?) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.fillMaxWidth()) {
            Row(modifier = Modifier.padding(8.dp)) {
                columns.forEach { column ->
                    Box(modifier = Modifier.weight(1f / columns.size), contentAlignment = Alignment.Center) {
                        Text(text = column.name, fontWeight = FontWeight.Bold)
                    }
                }
            }
            Divider(thickness = 1.dp)
            data?.forEach { row ->
                Row(modifier = Modifier.padding(8.dp)) {
                    columns.forEach { column ->
                        Box(modifier = Modifier.weight(1f / columns.size), contentAlignment = column.contentAlignment) {
                            column.renderCell(row)
                        }
                    }
                }
            }
            if (data == null || data.isEmpty()) {
                Box(modifier = Modifier.fillMaxWidth().padding(8.dp), contentAlignment = Alignment.Center) {
                    Text(text = "No data")
                }
            }
        }
    }
}