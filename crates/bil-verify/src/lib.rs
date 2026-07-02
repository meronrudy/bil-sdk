use bil_core::{BilStatus, ProfileId, ReceiptId};
use bil_signers::BilSignatureVerifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub status: BilStatus,
    pub receipt_id: Option<ReceiptId>,
    pub profile: Option<ProfileId>,
    pub checks: Vec<VerificationCheck>,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCheck {
    pub kind: VerificationCheckKind,
    pub status: BilStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationCheckKind {
    SchemaValid,
    CanonicalEncodingValid,
    CommitmentHashValid,
    SignatureValid,
    SignerKnown,
    EvidenceRefsPresent,
    MerkleRootValid,
    MerkleProofsValid,
    AuthorityRefsPresent,
    AuthorityBindingValid,
    PolicyRefsPresent,
    ReplayDeterministic,
    TimestampValid,
    AssuranceLevelValid,
    ProfileChecksPassed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub priority: FindingPriority,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingPriority {
    P0, // invalid proof / cannot trust artifact
    P1, // incomplete evidence / governance risk
    P2, // warning / non-production / metadata gap
}

pub struct VerificationEngine;

impl VerificationEngine {
    pub fn verify_receipt(receipt: &bil_ink::InkReceipt) -> VerificationReport {
        let mut checks = Vec::new();
        let mut findings = Vec::new();
        let mut overall_status = BilStatus::Pass;

        // 1. Check Signature
        let verifier = bil_signers::SoftwareDevSignatureVerifier;
        
        // Reconstruct canonical bytes of the receipt (excluding signature)
        // For now, we use the same placeholder logic as in issuance
        let mut receipt_copy = receipt.clone();
        receipt_copy.signature = bil_ink::SignatureBytes(vec![]);
        
        let receipt_json = serde_json::to_value(&receipt_copy).unwrap();
        let receipt_bil_value = bil_canonical::BilValue::try_from(&receipt_json).unwrap();
        let canonical_bytes = bil_canonical::encode_canonical(&receipt_bil_value).unwrap();

        // In a real implementation, we would look up the public key based on the signer ID
        // For now, we assume the signer ID is the hex-encoded public key for the software dev signer
        let public_key = bil_signers::PublicKeyRef(receipt.signer.0.clone());

        match verifier.verify_signature(&public_key, &canonical_bytes, &receipt.signature) {
            Ok(_) => {
                checks.push(VerificationCheck {
                    kind: VerificationCheckKind::SignatureValid,
                    status: BilStatus::Pass,
                });
            }
            Err(e) => {
                checks.push(VerificationCheck {
                    kind: VerificationCheckKind::SignatureValid,
                    status: BilStatus::Fail,
                });
                findings.push(Finding {
                    priority: FindingPriority::P0,
                    message: format!("Signature verification failed: {}", e),
                });
                overall_status = BilStatus::Fail;
            }
        }

        // 2. Check Assurance Level
        if receipt.assurance_level == bil_core::AssuranceLevel::L0SoftwareDev {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::AssuranceLevelValid,
                status: BilStatus::Warn,
            });
            findings.push(Finding {
                priority: FindingPriority::P2,
                message: "Receipt is signed with L0SoftwareDev key. Not suitable for production.".to_string(),
            });
            if overall_status == BilStatus::Pass {
                overall_status = BilStatus::Warn;
            }
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::AssuranceLevelValid,
                status: BilStatus::Pass,
            });
        }

        // 3. Check Event Refs
        if receipt.event_refs.is_empty() {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::EvidenceRefsPresent, // Reusing this for events for now
                status: BilStatus::Warn,
            });
            findings.push(Finding {
                priority: FindingPriority::P1,
                message: "Receipt contains no event references.".to_string(),
            });
            if overall_status == BilStatus::Pass {
                overall_status = BilStatus::Warn;
            }
        }

        // 4. Check Authority Refs
        if receipt.authority_refs.is_empty() {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::AuthorityRefsPresent,
                status: BilStatus::Warn,
            });
            findings.push(Finding {
                priority: FindingPriority::P1,
                message: "Receipt contains no authority references.".to_string(),
            });
            if overall_status == BilStatus::Pass {
                overall_status = BilStatus::Warn;
            }
        }

        // 5. Check Policy Refs
        if receipt.policy_refs.is_empty() {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::PolicyRefsPresent,
                status: BilStatus::Warn,
            });
            findings.push(Finding {
                priority: FindingPriority::P1,
                message: "Receipt contains no policy references.".to_string(),
            });
            if overall_status == BilStatus::Pass {
                overall_status = BilStatus::Warn;
            }
        }

        VerificationReport {
            status: overall_status,
            receipt_id: Some(receipt.receipt_id.clone()),
            profile: Some(receipt.profile.clone()),
            checks,
            findings,
        }
    }
}
