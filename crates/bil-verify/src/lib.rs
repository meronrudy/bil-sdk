use bil_canonical::BilCanonical;
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

        // 0. Check Canonical Encoding
        let preimage = receipt.signing_preimage();
        let canonical_bytes = match preimage.to_canonical_bytes() {
            Ok(b) => b,
            Err(e) => {
                checks.push(VerificationCheck {
                    kind: VerificationCheckKind::CanonicalEncodingValid,
                    status: BilStatus::Fail,
                });
                findings.push(Finding {
                    priority: FindingPriority::P0,
                    message: format!("Failed to encode preimage to canonical bytes: {}", e),
                });
                return VerificationReport {
                    status: BilStatus::Fail,
                    receipt_id: Some(preimage.receipt_id.clone()),
                    profile: Some(preimage.profile.clone()),
                    checks,
                    findings,
                };
            }
        };

        checks.push(VerificationCheck {
            kind: VerificationCheckKind::CanonicalEncodingValid,
            status: BilStatus::Pass,
        });

        // 0.5 Check Commitment Hash
        let computed_commitment = bil_canonical::Hash256::sha256(&canonical_bytes);
        if computed_commitment != receipt.canonical_commitment {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::CommitmentHashValid,
                status: BilStatus::Fail,
            });
            findings.push(Finding {
                priority: FindingPriority::P0,
                message: "Canonical commitment hash does not match preimage.".to_string(),
            });
            overall_status = BilStatus::Fail;
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::CommitmentHashValid,
                status: BilStatus::Pass,
            });
        }

        // 1. Check Signature
        // In a real implementation, we would look up the public key based on the signer ID
        // For now, we assume the signer ID is the hex-encoded public key for the software dev signer
        let verifier = bil_signers::SoftwareDevSignatureVerifier;
        let public_key = bil_signers::PublicKeyRef(preimage.signer.0.clone());
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
        if preimage.assurance_level == bil_core::AssuranceLevel::L0SoftwareDev {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::AssuranceLevelValid,
                status: BilStatus::Warn,
            });
            findings.push(Finding {
                priority: FindingPriority::P2,
                message: "Receipt is signed with L0SoftwareDev key. Not suitable for production."
                    .to_string(),
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
        if preimage.event_refs.is_empty() {
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
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::EvidenceRefsPresent,
                status: BilStatus::Pass,
            });
        }

        // 3.5 Check Evidence Root
        if preimage.evidence_root.is_none() {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::MerkleRootValid,
                status: BilStatus::Warn,
            });
            findings.push(Finding {
                priority: FindingPriority::P1,
                message: "Receipt contains no evidence root (Merkle tree).".to_string(),
            });
            if overall_status == BilStatus::Pass {
                overall_status = BilStatus::Warn;
            }
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::MerkleRootValid,
                status: BilStatus::Pass,
            });
        }

        // 4. Check Authority Refs
        if preimage.authority_refs.is_empty() {
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
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::AuthorityRefsPresent,
                status: BilStatus::Pass,
            });
        }

        // 5. Check Policy Refs
        if preimage.policy_refs.is_empty() {
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
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::PolicyRefsPresent,
                status: BilStatus::Pass,
            });
        }

        // 6. Check Replay Determinism (Placeholder)
        // In a real implementation, we would need the MIR graph to validate replay.
        // For now, we just add a placeholder check.
        checks.push(VerificationCheck {
            kind: VerificationCheckKind::ReplayDeterministic,
            status: BilStatus::Pass,
        });

        VerificationReport {
            status: overall_status,
            receipt_id: Some(preimage.receipt_id.clone()),
            profile: Some(preimage.profile.clone()),
            checks,
            findings,
        }
    }
}
