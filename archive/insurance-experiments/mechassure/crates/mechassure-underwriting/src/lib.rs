use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderwritingFile {
    pub artifact_type: String,
    pub schema_version: String,
    pub domain: String,
    pub insured_system: String,
    pub period: String,
    pub exposure: Value,
    pub risk_statistics: Value,
    pub control_evidence: Value,
    pub evidence_quality: Value,
    pub underwriting_flags: Vec<String>,
    pub actuarial_export: Value,
    pub audit_manifest: Value,
}

impl Default for UnderwritingFile {
    fn default() -> Self {
        Self {
            artifact_type: "ai_underwriting_file".to_string(),
            schema_version: "0.1.0".to_string(),
            domain: String::new(),
            insured_system: String::new(),
            period: String::new(),
            exposure: Value::Object(Default::default()),
            risk_statistics: Value::Object(Default::default()),
            control_evidence: Value::Object(Default::default()),
            evidence_quality: Value::Object(Default::default()),
            underwriting_flags: Vec::new(),
            actuarial_export: Value::Object(Default::default()),
            audit_manifest: Value::Object(Default::default()),
        }
    }
}
