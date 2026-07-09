//! Version marker for feature bundle schemas.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureVersion {
    pub major: u16,
    pub minor: u16,
}

impl Default for FeatureVersion {
    fn default() -> Self {
        Self { major: 1, minor: 0 }
    }
}
