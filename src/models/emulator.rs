//! Android emulators (AVDs) and their launch parameters.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmulatorParams {
    pub no_snapshot_load: bool,
    pub writeable_system: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Emulator {
    pub id: u64,
    pub name: String,
    pub params: EmulatorParams,
}
