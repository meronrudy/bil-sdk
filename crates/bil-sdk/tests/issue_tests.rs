use bil_core::AssuranceLevel;
use bil_ink::CapabilityCode;
use bil_mock::SyntheticProfile;
use bil_sdk::{Bil, DemoProfile};

#[test]
fn issue_populates_evidence_root_when_evidence_exists() {
    let graph = Bil::mock(SyntheticProfile::BankBranch)
        .with_seed(2026)
        .include_ai_assist(true)
        .include_human_review(true)
        .build()
        .unwrap();

    assert!(!graph.evidence.is_empty());

    let artifact = Bil::issue(graph, CapabilityCode("AssuranceReceipt".to_string())).unwrap();
    let receipt = artifact.receipt;

    assert_eq!(receipt.preimage.capability.0, "AssuranceReceipt");
    assert_eq!(
        receipt.preimage.assurance_level,
        AssuranceLevel::L0SoftwareDev
    );
    assert!(!receipt.signature.0.is_empty());
    assert!(!receipt.preimage.signer.0.is_empty());
    assert!(!receipt.preimage.event_refs.is_empty());
    assert!(!receipt.preimage.authority_refs.is_empty());
    assert!(!receipt.preimage.policy_refs.is_empty());
    assert!(receipt.preimage.evidence_root.is_some());
}

#[test]
fn evidence_mutation_changes_merkle_root() {
    let graph_a = Bil::mock(SyntheticProfile::BankBranch)
        .with_seed(2026)
        .build()
        .unwrap();

    let mut graph_b = graph_a.clone();
    graph_b.evidence[0].hash = bil_canonical::Hash256::sha256(b"mutated");

    let root_a = bil_ink::merkle::MerkleTree::build(&graph_a.evidence)
        .unwrap()
        .unwrap()
        .root;
    let root_b = bil_ink::merkle::MerkleTree::build(&graph_b.evidence)
        .unwrap()
        .unwrap()
        .root;

    assert_ne!(root_a.0, root_b.0);
}

#[test]
fn verifier_fails_when_canonical_commitment_is_mutated() {
    let demo = Bil::demo(DemoProfile::BankBranch).unwrap();
    let mut receipt = demo.receipt().unwrap();

    receipt.canonical_commitment = bil_canonical::Hash256::zero();

    let report = Bil::verify(&receipt).unwrap();

    assert_eq!(report.status, bil_core::BilStatus::Fail);
}

#[test]
fn verifier_accepts_issued_receipt_signature() {
    let demo = Bil::demo(DemoProfile::BankBranch).unwrap();
    let receipt = demo.receipt().unwrap();

    let report = Bil::verify(&receipt).unwrap();

    // The overall status might be Warn due to L0SoftwareDev, but it shouldn't be Fail
    assert_ne!(
        report.status,
        bil_core::BilStatus::Fail,
        "Verification failed with findings: {:?}",
        report.findings
    );
    assert!(report.checks.iter().any(|c| c.kind
        == bil_verify::VerificationCheckKind::SignatureValid
        && c.status == bil_core::BilStatus::Pass));
}

#[test]
fn demo_receipt_uses_issue_path() {
    let demo = Bil::demo(DemoProfile::BankBranch).unwrap();
    let receipt = demo.receipt().unwrap();

    assert_eq!(receipt.preimage.capability.0, "demo.receipt");
    assert_eq!(
        receipt.preimage.assurance_level,
        AssuranceLevel::L0SoftwareDev
    );
    assert!(!receipt.signature.0.is_empty());
}
