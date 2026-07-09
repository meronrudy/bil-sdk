//! Event envelope contracts.

use crate::{MachineId, MonotonicMicros};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemetryFrame {
    pub machine_id: MachineId,
    pub timestamp: MonotonicMicros,
    pub raw_data: Vec<u8>,
}

impl TelemetryFrame {
    pub fn new(machine_id: MachineId, timestamp: MonotonicMicros, raw_data: Vec<u8>) -> Self {
        Self {
            machine_id,
            timestamp,
            raw_data,
        }
    }
}
