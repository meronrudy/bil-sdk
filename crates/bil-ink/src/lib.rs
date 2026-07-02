use bil_canonical::Hash256;
use bil_core::{AssuranceLevel, AuthorityRef, EventId, EvidenceRef, PolicyRef, ProfileId, ReceiptId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityCode(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuerRef(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectRef(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerRef(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBytes(pub Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleRoot(pub Hash256);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InkReceipt {
    pub receipt_id: ReceiptId,
    pub capability: CapabilityCode,
    pub profile: ProfileId,
    pub issuer: IssuerRef,
    pub subject: SubjectRef,

    pub mir_commitment: Hash256,
    pub evidence_root: Option<MerkleRoot>,
    pub event_refs: Vec<EventId>,
    pub authority_refs: Vec<AuthorityRef>,
    pub policy_refs: Vec<PolicyRef>,

    pub canonical_commitment: Hash256,
    pub signer: SignerRef,
    pub signature: SignatureBytes,
    pub timestamp_ms: i64,
    pub assurance_level: AssuranceLevel,
}

pub mod merkle {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct MerkleTree {
        pub leaves: Vec<MerkleLeaf>,
        pub root: MerkleRoot,
    }

    #[derive(Debug, Clone)]
    pub struct MerkleLeaf {
        pub index: u64,
        pub evidence_ref: EvidenceRef,
        pub hash: Hash256,
    }

    #[derive(Debug, Clone)]
    pub struct MerkleProof {
        pub leaf: MerkleLeaf,
        pub siblings: Vec<MerkleSibling>,
        pub root: MerkleRoot,
    }

    #[derive(Debug, Clone)]
    pub struct MerkleSibling {
        pub hash: Hash256,
        pub direction: MerkleDirection,
    }

    #[derive(Debug, Clone)]
    pub enum MerkleDirection {
        Left,
        Right,
    }
}
