use bil_canonical::{encode_canonical, BilCanonical, BilValue, Hash256};
use bil_core::BilStatus;
use bil_ink::CapabilityCode;
use bil_sdk::{Bil, SyntheticProfile};
use bil_verify::VerificationCheckKind;

pub fn run_conformance_group(group: &str) -> Result<(), String> {
    match group {
        "all" => {
            canonical_encoding_case()?;
            receipt_roundtrip_case()?;
            signature_verification_case()?;
            merkle_proof_case()?;
            verification_semantics_case()?;
        }
        "canonical-encoding" => canonical_encoding_case()?,
        "receipt-roundtrip" => receipt_roundtrip_case()?,
        "signature-verification" => signature_verification_case()?,
        "merkle-proof" | "merkle-root" => merkle_proof_case()?,
        "verification-semantics" => verification_semantics_case()?,
        _ => return Err(format!("Unknown conformance group: {}", group)),
    }

    println!("Conformance group '{}' passed.", group);
    Ok(())
}

fn canonical_encoding_case() -> Result<(), String> {
    let value = BilValue::Map(vec![
        (BilValue::Text("b".to_string()), BilValue::Integer(2)),
        (BilValue::Text("a".to_string()), BilValue::Integer(1)),
    ]);
    let encoded = encode_canonical(&value).map_err(|e| e.to_string())?;
    let expected = vec![0xa2, 0x61, 0x61, 0x01, 0x61, 0x62, 0x02];

    if encoded == expected {
        Ok(())
    } else {
        Err("canonical map encoding did not match expected vector".to_string())
    }
}

fn receipt_roundtrip_case() -> Result<(), String> {
    let receipt = issued_receipt()?;
    let preimage_bytes = receipt
        .signing_preimage()
        .to_canonical_bytes()
        .map_err(|e| e.to_string())?;
    let expected_commitment = Hash256::sha256(&preimage_bytes);

    if receipt.canonical_commitment != expected_commitment {
        return Err("receipt commitment does not match explicit preimage".to_string());
    }

    let encoded = serde_json::to_string(&receipt).map_err(|e| e.to_string())?;
    let decoded: bil_ink::InkReceipt = serde_json::from_str(&encoded).map_err(|e| e.to_string())?;

    if decoded.canonical_commitment == receipt.canonical_commitment {
        Ok(())
    } else {
        Err("receipt JSON roundtrip changed canonical commitment".to_string())
    }
}

fn signature_verification_case() -> Result<(), String> {
    let receipt = issued_receipt()?;
    let report = Bil::verify(&receipt).map_err(|e| e.to_string())?;

    if report.status == BilStatus::Fail {
        return Err("issued receipt verification failed".to_string());
    }

    has_passed_check(&report, VerificationCheckKind::SignatureValid)
        .then_some(())
        .ok_or_else(|| "signature check did not pass".to_string())
}

fn merkle_proof_case() -> Result<(), String> {
    let graph_a = Bil::mock(SyntheticProfile::HumanOverride)
        .with_seed(2026)
        .build()
        .map_err(|e| e.to_string())?;
    let mut graph_b = graph_a.clone();
    graph_b.evidence[0].hash = Hash256::sha256(b"mutated");

    let tree_a = bil_ink::merkle::MerkleTree::build(&graph_a.evidence)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "expected evidence Merkle tree".to_string())?;
    let tree_b = bil_ink::merkle::MerkleTree::build(&graph_b.evidence)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "expected mutated evidence Merkle tree".to_string())?;

    if tree_a.root.0 == tree_b.root.0 {
        return Err("mutating evidence did not change Merkle root".to_string());
    }

    let proof = tree_a
        .generate_proof(0)
        .ok_or_else(|| "expected proof for first evidence leaf".to_string())?;

    proof
        .verify()
        .then_some(())
        .ok_or_else(|| "generated Merkle proof did not verify".to_string())
}

fn verification_semantics_case() -> Result<(), String> {
    let receipt = issued_receipt()?;
    let report = Bil::verify(&receipt).map_err(|e| e.to_string())?;

    for kind in [
        VerificationCheckKind::CanonicalEncodingValid,
        VerificationCheckKind::CommitmentHashMatches,
        VerificationCheckKind::SignatureValid,
        VerificationCheckKind::RequiredReferencePresent,
        VerificationCheckKind::ProfileDeclared,
        VerificationCheckKind::ReceiptEnvelopeValid,
        VerificationCheckKind::SchemaValid,
    ] {
        if !has_passed_check(&report, kind.clone()) {
            return Err(format!("structural check did not pass: {:?}", kind));
        }
    }

    let mut mutated = receipt;
    mutated.canonical_commitment = Hash256::zero();
    let mutated_report = Bil::verify(&mutated).map_err(|e| e.to_string())?;

    if mutated_report.status == BilStatus::Fail {
        Ok(())
    } else {
        Err("mutated commitment did not fail verification".to_string())
    }
}

fn issued_receipt() -> Result<bil_ink::InkReceipt, String> {
    let graph = Bil::mock(SyntheticProfile::HumanOverride)
        .with_seed(2026)
        .build()
        .map_err(|e| e.to_string())?;
    let artifact = Bil::issue(graph, CapabilityCode("conformance.receipt".to_string()))
        .map_err(|e| e.to_string())?;
    Ok(artifact.receipt)
}

fn has_passed_check(report: &bil_verify::VerificationReport, kind: VerificationCheckKind) -> bool {
    report
        .checks
        .iter()
        .any(|check| check.kind == kind && check.status == BilStatus::Pass)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_conformance_groups_pass() {
        run_conformance_group("all").unwrap();
    }

    #[test]
    fn individual_conformance_groups_pass() {
        for group in [
            "canonical-encoding",
            "receipt-roundtrip",
            "signature-verification",
            "merkle-proof",
            "verification-semantics",
        ] {
            run_conformance_group(group).unwrap();
        }
    }

    #[test]
    fn unknown_group_fails() {
        assert!(run_conformance_group("profile-specific").is_err());
    }
}
