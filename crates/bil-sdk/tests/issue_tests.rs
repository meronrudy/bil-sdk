use bil_core::AssuranceLevel;
use bil_ink::CapabilityCode;
use bil_mock::SyntheticProfile;
use bil_sdk::{Bil, DemoProfile};

#[test]
fn issue_bank_branch_mock_produces_signed_receipt() {
    let graph = Bil::mock(SyntheticProfile::BankBranch)
        .with_seed(2026)
        // .include_ai_assist(true) // TODO: add these builder methods
        // .include_human_review(true)
        .build()
        .unwrap();

    let artifact = Bil::issue(graph, CapabilityCode("AssuranceReceipt".to_string())).unwrap();
    let receipt = artifact.receipt;

    assert_eq!(receipt.capability.0, "AssuranceReceipt");
    assert_eq!(receipt.assurance_level, AssuranceLevel::L0SoftwareDev);
    assert!(!receipt.signature.0.is_empty());
    assert!(!receipt.signer.0.is_empty());
    assert!(!receipt.event_refs.is_empty());
    assert!(!receipt.authority_refs.is_empty());
    assert!(!receipt.policy_refs.is_empty());
}

#[test]
fn demo_receipt_uses_issue_path() {
    let demo = Bil::demo(DemoProfile::BankBranch).unwrap();
    let receipt = demo.receipt().unwrap();

    assert_eq!(receipt.capability.0, "demo.receipt");
    assert_eq!(receipt.assurance_level, AssuranceLevel::L0SoftwareDev);
    assert!(!receipt.signature.0.is_empty());
}
