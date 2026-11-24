//! Saved deep links, opened on the device with `am start`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DeepLink {
    pub id: u64,
    pub name: String,
    pub link: String,
    /// The app that should open the link. Empty lets Android choose.
    pub package: String,
}
