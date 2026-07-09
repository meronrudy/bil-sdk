//! Minimal internal control-plane API for alpha batch jobs and artifact retrieval.

use actuarial::rate_filing::RateFilingArtifact;
use contracts::{EventId, SiteTime};
use risk_layer::{RiskLayer, RiskLayerInput, RiskLayerReport};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPlaneConfig {
    pub data_dir: PathBuf,
}

impl Default for ControlPlaneConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from(".aissurance-alpha"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BatchJobStatus {
    Submitted,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredArtifacts {
    pub input_path: PathBuf,
    pub report_path: PathBuf,
    pub filing_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJobRecord {
    pub job_id: String,
    pub submitted_at: SiteTime,
    pub status: BatchJobStatus,
    pub frames_ingested: usize,
    pub claims: usize,
    pub artifacts: StoredArtifacts,
}

#[derive(Debug, Error)]
pub enum ControlPlaneError {
    #[error("risk layer execution failed: {0}")]
    Risk(#[from] risk_layer::RiskLayerError),
    #[error("io failure: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization failure: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub trait ArtifactStore {
    fn persist(
        &self,
        job_id: &str,
        input: &RiskLayerInput,
        report: &RiskLayerReport,
        filing: &RateFilingArtifact,
    ) -> Result<StoredArtifacts, ControlPlaneError>;

    fn load_job(&self, job_id: &str) -> Result<BatchJobRecord, ControlPlaneError>;
    fn load_report(&self, job_id: &str) -> Result<RiskLayerReport, ControlPlaneError>;
}

#[derive(Debug, Clone)]
pub struct FileArtifactStore {
    root: PathBuf,
}

impl FileArtifactStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn job_dir(&self, job_id: &str) -> PathBuf {
        self.root.join("jobs").join(job_id)
    }

    fn ensure_root(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(self.root.join("jobs"))
    }
}

impl ArtifactStore for FileArtifactStore {
    fn persist(
        &self,
        job_id: &str,
        input: &RiskLayerInput,
        report: &RiskLayerReport,
        filing: &RateFilingArtifact,
    ) -> Result<StoredArtifacts, ControlPlaneError> {
        self.ensure_root()?;
        let job_dir = self.job_dir(job_id);
        fs::create_dir_all(&job_dir)?;

        let input_path = job_dir.join("input.json");
        let report_path = job_dir.join("report.json");
        let filing_path = job_dir.join("filing.json");

        fs::write(&input_path, serde_json::to_vec_pretty(input)?)?;
        fs::write(&report_path, serde_json::to_vec_pretty(report)?)?;
        fs::write(&filing_path, filing.to_json()?)?;

        Ok(StoredArtifacts {
            input_path,
            report_path,
            filing_path,
        })
    }

    fn load_job(&self, job_id: &str) -> Result<BatchJobRecord, ControlPlaneError> {
        let metadata_path = self.job_dir(job_id).join("job.json");
        let bytes = fs::read(metadata_path)?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    fn load_report(&self, job_id: &str) -> Result<RiskLayerReport, ControlPlaneError> {
        let report_path = self.job_dir(job_id).join("report.json");
        let bytes = fs::read(report_path)?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[derive(Debug, Clone)]
pub struct ControlPlane<S = FileArtifactStore> {
    risk_layer: RiskLayer,
    store: S,
}

impl ControlPlane<FileArtifactStore> {
    pub fn new(config: ControlPlaneConfig, risk_layer: RiskLayer) -> Self {
        Self {
            risk_layer,
            store: FileArtifactStore::new(config.data_dir),
        }
    }
}

impl<S: ArtifactStore> ControlPlane<S> {
    pub fn with_store(risk_layer: RiskLayer, store: S) -> Self {
        Self { risk_layer, store }
    }

    pub fn submit_batch(&self, input: RiskLayerInput) -> Result<BatchJobRecord, ControlPlaneError> {
        let report = self.risk_layer.run_batch(input.clone())?;
        let job_id = event_id_string(EventId::default());
        let filing = report.to_filing_artifact();
        let artifacts = self.store.persist(&job_id, &input, &report, &filing)?;
        let record = BatchJobRecord {
            job_id: job_id.clone(),
            submitted_at: SiteTime::now(),
            status: BatchJobStatus::Completed,
            frames_ingested: report.frames_ingested,
            claims: report.claims,
            artifacts,
        };
        self.write_metadata(&record)?;
        Ok(record)
    }

    pub fn job_status(&self, job_id: &str) -> Result<BatchJobRecord, ControlPlaneError> {
        self.store.load_job(job_id)
    }

    pub fn report(&self, job_id: &str) -> Result<RiskLayerReport, ControlPlaneError> {
        self.store.load_report(job_id)
    }

    fn write_metadata(&self, record: &BatchJobRecord) -> Result<(), ControlPlaneError> {
        let path = record
            .artifacts
            .report_path
            .parent()
            .unwrap_or(Path::new("."))
            .join("job.json");
        fs::write(path, serde_json::to_vec_pretty(record)?)?;
        Ok(())
    }
}

fn event_id_string(id: EventId) -> String {
    id.as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use contracts::{MachineId, MonotonicMicros, TelemetryFrame};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn persists_and_reads_back_jobs() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string();
        let root = std::env::temp_dir().join(format!("aissurance-control-plane-{suffix}"));
        let control_plane = ControlPlane::new(
            ControlPlaneConfig {
                data_dir: root.clone(),
            },
            RiskLayer::default(),
        );

        let input = RiskLayerInput::new(
            vec![TelemetryFrame::new(
                MachineId::test_id(1),
                MonotonicMicros::new(1),
                b"{\"type\":\"load\",\"load_percentage\":0.5}".to_vec(),
            )],
            vec![],
        );

        let record = control_plane.submit_batch(input).unwrap();
        let loaded = control_plane.job_status(&record.job_id).unwrap();
        let report = control_plane.report(&record.job_id).unwrap();

        assert_eq!(loaded.status, BatchJobStatus::Completed);
        assert_eq!(report.frames_ingested, 1);

        let _ = fs::remove_dir_all(root);
    }
}
