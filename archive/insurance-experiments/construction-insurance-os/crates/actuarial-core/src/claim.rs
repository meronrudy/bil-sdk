use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use ctw_core::{ClaimId, PolicyId, MachineId, SiteId};
use crate::peril::Peril;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claim {
    pub id: ClaimId,
    pub policy_id: PolicyId,
    pub accident_date: NaiveDate,
    pub report_date: NaiveDate,
    pub peril: Peril,
    pub incurred: f64,
    pub paid: f64,
    pub case_reserve: f64,
    pub alae: f64,
    pub status: ClaimStatus,
    pub machine_id: Option<MachineId>,
    pub site_id: Option<SiteId>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ClaimStatus {
    Open,
    Closed,
    Reopened,
    Subrogated,
}

impl Claim {
    /// Total incurred = paid + outstanding reserve + ALAE.
    pub fn total_incurred(&self) -> f64 {
        self.paid + self.case_reserve + self.alae
    }
}
