//! Machine health and maintenance state.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::{Hours, MachineId, MonotonicMicros, HealthStatus};

/// Snapshot of a machine's health at a point in time.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HealthSnapshot {
    pub machine_id: MachineId,
    pub timestamp: MonotonicMicros,
    pub overall_status: HealthStatus,
    pub engine_hours: Hours,
    pub hours_until_maintenance: Hours,
    pub active_fault_count: u32,
    pub deferred_maintenance_hours: Hours,
}

impl HealthSnapshot {
    /// Is maintenance overdue?
    #[must_use]
    pub fn maintenance_overdue(&self) -> bool {
        self.hours_until_maintenance.raw() < 0.0
    }
}
