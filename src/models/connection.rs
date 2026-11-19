//! Saved remote connections.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Connection {
    pub id: u64,
    pub name: String,
    pub address: String,
}
