use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use ctw_core::PolicyId;
use crate::coverage::CoverageForm;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    pub id: PolicyId,
    pub effective_date: NaiveDate,
    pub expiration_date: NaiveDate,
    pub coverage: CoverageForm,
    pub written_premium: f64,
    pub written_exposure: f64,
    pub status: PolicyStatus,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PolicyStatus {
    Quoted,
    Bound,
    InForce,
    Expired,
    Cancelled,
}
