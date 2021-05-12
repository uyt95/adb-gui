package components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.width
import androidx.compose.material.Button
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.vectorXmlResource
import androidx.compose.ui.unit.dp

@Composable
fun vectorIconButton(modifier: Modifier? = null, name: String, contentDescription: String, enabled: Boolean = true, onClick: () -> Unit) {
    Button(
        modifier = if (modifier == null) Modifier.width(48.dp) else Modifier.width(48.dp).then(modifier),
        contentPadding = PaddingValues(4.dp),
        enabled = enabled,
        onClick = onClick
    ) {
        Image(imageVector = vectorXmlResource("icons/$name.xml"), contentDescription = contentDescription)
    }
}