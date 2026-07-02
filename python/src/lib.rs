use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

use bil_sdk::{Bil, DemoProfile, DemoRun};
use bil_verify::VerificationReport;

#[pyclass]
pub struct PyDemoRun {
    inner: DemoRun,
}

#[pymethods]
impl PyDemoRun {
    fn verify(&self) -> PyResult<PyVerificationReport> {
        let report = self
            .inner
            .verify()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(PyVerificationReport { inner: report })
    }
}

#[pyclass]
pub struct PyVerificationReport {
    inner: VerificationReport,
}

#[pymethods]
impl PyVerificationReport {
    fn status(&self) -> String {
        format!("{:?}", self.inner.status)
    }

    fn markdown(&self) -> String {
        "Markdown report".to_string()
    }

    fn display(&self) {
        println!("Verification Report: {:?}", self.inner.status);
    }
}

#[pyfunction]
fn demo(profile: &str) -> PyResult<PyDemoRun> {
    let profile = match profile {
        "bank_branch" => DemoProfile::BankBranch,
        "loan_decision" => DemoProfile::LoanDecision,
        "ai_assurance" => DemoProfile::AiAssurance,
        _ => return Err(PyValueError::new_err("unknown profile")),
    };

    let run = Bil::demo(profile).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(PyDemoRun { inner: run })
}

#[pymodule]
fn bankabil(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(demo, m)?)?;
    m.add_class::<PyDemoRun>()?;
    m.add_class::<PyVerificationReport>()?;
    Ok(())
}
