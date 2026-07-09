#![cfg(feature = "aissurance-local")]

use serde_json::Value;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn aissurance_platform_demo_emits_receipts_and_manifest() {
    let data_dir = tempdir().unwrap();
    let out_dir = tempdir().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_bil-cli"))
        .args([
            "aissurance",
            "platform-demo",
            "--data-dir",
            data_dir.path().to_str().unwrap(),
            "--out-dir",
            out_dir.path().to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(!manifest["run_id"].as_str().unwrap().is_empty());
    assert!(!manifest["job_id"].as_str().unwrap().is_empty());

    for key in [
        "planner_receipt",
        "safety_receipt",
        "risk_receipt",
        "run_receipt",
    ] {
        assert!(!manifest[key]["receipt_id"].as_str().unwrap().is_empty());
        assert!(fs::metadata(manifest[key]["receipt_path"].as_str().unwrap()).is_ok());
        assert!(fs::metadata(manifest[key]["verification_report_path"].as_str().unwrap()).is_ok());
    }

    for key in ["input_path", "report_path", "filing_path", "job_path"] {
        assert!(fs::metadata(manifest["underlying_artifacts"][key].as_str().unwrap()).is_ok());
    }
}
