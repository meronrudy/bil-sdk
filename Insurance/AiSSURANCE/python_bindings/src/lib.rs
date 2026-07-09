use contracts::{MachineId, MonotonicMicros, TelemetryFrame};
use control_plane::{ControlPlane, ControlPlaneConfig};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use risk_layer::{RiskLayer, RiskLayerInput};
use std::sync::Arc;

#[pyclass]
struct PyControlPlane {
    inner: Arc<ControlPlane>,
}

#[pymethods]
impl PyControlPlane {
    #[new]
    fn new(data_dir: Option<String>) -> PyResult<Self> {
        let config = if let Some(dir) = data_dir {
            ControlPlaneConfig {
                data_dir: std::path::PathBuf::from(dir),
            }
        } else {
            ControlPlaneConfig::default()
        };
        let risk_layer = RiskLayer::default();
        let inner = Arc::new(ControlPlane::new(config, risk_layer));
        Ok(Self { inner })
    }

    fn submit_batch(&self, py: Python, input_json: String) -> PyResult<String> {
        let input: RiskLayerInput = serde_json::from_str(&input_json)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        let record = self
            .inner
            .submit_batch(input)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let record_json = serde_json::to_string(&record)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(record_json)
    }

    fn job_status(&self, py: Python, job_id: String) -> PyResult<String> {
        let record = self
            .inner
            .job_status(&job_id)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let record_json = serde_json::to_string(&record)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(record_json)
    }

    fn report(&self, py: Python, job_id: String) -> PyResult<String> {
        let report = self
            .inner
            .report(&job_id)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let report_json = serde_json::to_string(&report)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(report_json)
    }
}

#[pymodule]
fn aissurance(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyControlPlane>()?;
    Ok(())
}
