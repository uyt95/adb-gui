package models

import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment

class TableColumn<T>(
    val name: String,
    val contentAlignment: Alignment = Alignment.CenterStart,
    val renderCell: @Composable (row: T) -> Unit,
)
