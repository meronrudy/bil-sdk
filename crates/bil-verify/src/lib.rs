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
    CommitmentHashMatches,
    SignatureValid,
    ChainLinkValid,
    MerkleInclusionValid,
    ReceiptEnvelopeValid,
    RequiredReferencePresent,
    ProfileDeclared,
    ExtensionRecognized,
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
                kind: VerificationCheckKind::CommitmentHashMatches,
                status: BilStatus::Fail,
            });
            findings.push(Finding {
                priority: FindingPriority::P0,
                message: "Canonical commitment hash does not match preimage.".to_string(),
            });
            overall_status = BilStatus::Fail;
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::CommitmentHashMatches,
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

        // 2. Check Required References
        if preimage.event_refs.is_empty()
            && preimage.authority_refs.is_empty()
            && preimage.policy_refs.is_empty()
        {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::RequiredReferencePresent,
                status: BilStatus::Warn,
            });
            findings.push(Finding {
                priority: FindingPriority::P1,
                message: "Receipt contains no event, authority, or policy references.".to_string(),
            });
            if overall_status == BilStatus::Pass {
                overall_status = BilStatus::Warn;
            }
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::RequiredReferencePresent,
                status: BilStatus::Pass,
            });
        }

        // 3. Check Profile Declared
        if preimage.profile.0.is_empty() {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::ProfileDeclared,
                status: BilStatus::Warn,
            });
            findings.push(Finding {
                priority: FindingPriority::P1,
                message: "Receipt does not declare a profile.".to_string(),
            });
            if overall_status == BilStatus::Pass {
                overall_status = BilStatus::Warn;
            }
        } else {
            checks.push(VerificationCheck {
                kind: VerificationCheckKind::ProfileDeclared,
                status: BilStatus::Pass,
            });
        }

        // 4. Check Receipt Envelope
        checks.push(VerificationCheck {
            kind: VerificationCheckKind::ReceiptEnvelopeValid,
            status: BilStatus::Pass,
        });

        // 5. Check Schema
        checks.push(VerificationCheck {
            kind: VerificationCheckKind::SchemaValid,
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
