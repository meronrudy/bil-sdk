use bil_canonical::{BilCanonical, BilValue, CanonicalError, Hash256};
use bil_core::{
    AssuranceLevel, AuthorityRef, EventId, EvidenceRef, PolicyRef, ProfileId, ReceiptId,
};
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
pub struct InkReceiptPreimage {
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

    pub signer: SignerRef,
    pub timestamp_ms: i64,
    pub assurance_level: AssuranceLevel,
}

impl BilCanonical for InkReceiptPreimage {
    fn to_canonical_value(&self) -> Result<BilValue, CanonicalError> {
        let json =
            serde_json::to_value(self).map_err(|e| CanonicalError::Encoding(e.to_string()))?;
        BilValue::try_from(&json)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InkReceipt {
    #[serde(flatten)]
    pub preimage: InkReceiptPreimage,
    pub canonical_commitment: Hash256,
    pub signature: SignatureBytes,
}

impl InkReceipt {
    /// Returns the explicit commitment preimage.
    ///
    /// `canonical_commitment` and `signature` are intentionally excluded from
    /// this value; they must never be fields inside the payload they commit to.
    pub fn signing_preimage(&self) -> InkReceiptPreimage {
        self.preimage.clone()
    }

    pub fn evidence_root(&self) -> Option<&MerkleRoot> {
        self.preimage.evidence_root.as_ref()
    }
}

pub mod merkle {
    use super::*;
    use bil_canonical::Hash256;

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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EvidenceLeafPreimage {
        pub evidence_id: EvidenceRef,
        pub evidence_hash: Hash256,
        pub kind: Option<String>,
    }

    impl BilCanonical for EvidenceLeafPreimage {
        fn to_canonical_value(&self) -> Result<BilValue, CanonicalError> {
            let mut map = vec![
                (
                    BilValue::Text("evidence_id".to_string()),
                    BilValue::Text(self.evidence_id.0.clone()),
                ),
                (
                    BilValue::Text("evidence_hash".to_string()),
                    BilValue::Bytes(self.evidence_hash.0.to_vec()),
                ),
            ];
            if let Some(kind) = &self.kind {
                map.push((
                    BilValue::Text("kind".to_string()),
                    BilValue::Text(kind.clone()),
                ));
            }
            Ok(BilValue::Map(map))
        }
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum MerkleDirection {
        Left,
        Right,
    }

    impl MerkleTree {
        pub fn build(
            evidence_nodes: &[bil_mir::EvidenceRefNode],
        ) -> Result<Option<Self>, CanonicalError> {
            if evidence_nodes.is_empty() {
                return Ok(None);
            }

            let mut preimages: Vec<EvidenceLeafPreimage> = evidence_nodes
                .iter()
                .map(|node| EvidenceLeafPreimage {
                    evidence_id: node.id.clone(),
                    evidence_hash: node.hash.clone(),
                    kind: node.kind.clone(),
                })
                .collect();

            // Sort leaves deterministically by their canonical bytes
            preimages.sort_by(|a, b| {
                let a_bytes = a.to_canonical_bytes().unwrap_or_default();
                let b_bytes = b.to_canonical_bytes().unwrap_or_default();
                a_bytes.cmp(&b_bytes)
            });

            let mut leaves = Vec::new();
            let mut current_level = Vec::new();

            for (index, preimage) in preimages.into_iter().enumerate() {
                let mut hash_input = Vec::new();
                hash_input.extend_from_slice(b"BIL_EVIDENCE_LEAF_V1");
                hash_input.extend_from_slice(&preimage.to_canonical_bytes()?);

                let hash = Hash256::sha256(&hash_input);
                let leaf = MerkleLeaf {
                    index: index as u64,
                    evidence_ref: preimage.evidence_id,
                    hash: hash.clone(),
                };
                leaves.push(leaf);
                current_level.push(hash);
            }

            let root = Self::compute_root(&current_level);

            Ok(Some(Self {
                leaves,
                root: MerkleRoot(root),
            }))
        }

        fn compute_root(level: &[Hash256]) -> Hash256 {
            if level.is_empty() {
                return Hash256([0; 32]);
            }
            if level.len() == 1 {
                return level[0].clone();
            }

            let mut next_level = Vec::new();
            for chunk in level.chunks(2) {
                let mut combined = Vec::new();
                combined.extend_from_slice(b"BIL_MERKLE_NODE_V1");

                if chunk.len() == 2 {
                    combined.extend_from_slice(&chunk[0].0);
                    combined.extend_from_slice(&chunk[1].0);
                } else {
                    // Duplicate the last element if odd number of nodes
                    combined.extend_from_slice(&chunk[0].0);
                    combined.extend_from_slice(&chunk[0].0);
                }
                next_level.push(Hash256::sha256(&combined));
            }

            Self::compute_root(&next_level)
        }

        pub fn generate_proof(&self, index: usize) -> Option<MerkleProof> {
            if index >= self.leaves.len() {
                return None;
            }

            let leaf = self.leaves[index].clone();
            let mut siblings = Vec::new();

            let mut current_index = index;
            let mut current_level: Vec<Hash256> =
                self.leaves.iter().map(|l| l.hash.clone()).collect();

            while current_level.len() > 1 {
                let is_right_child = !current_index.is_multiple_of(2);
                let sibling_index = if is_right_child {
                    current_index - 1
                } else {
                    current_index + 1
                };

                let sibling_hash = if sibling_index < current_level.len() {
                    current_level[sibling_index].clone()
                } else {
                    // Duplicate last element if odd
                    current_level[current_index].clone()
                };

                siblings.push(MerkleSibling {
                    hash: sibling_hash,
                    direction: if is_right_child {
                        MerkleDirection::Left
                    } else {
                        MerkleDirection::Right
                    },
                });

                let mut next_level = Vec::new();
                for chunk in current_level.chunks(2) {
                    let mut combined = Vec::new();
                    combined.extend_from_slice(b"BIL_MERKLE_NODE_V1");
                    if chunk.len() == 2 {
                        combined.extend_from_slice(&chunk[0].0);
                        combined.extend_from_slice(&chunk[1].0);
                    } else {
                        combined.extend_from_slice(&chunk[0].0);
                        combined.extend_from_slice(&chunk[0].0);
                    }
                    next_level.push(Hash256::sha256(&combined));
                }
                current_level = next_level;
                current_index /= 2;
            }

            Some(MerkleProof {
                leaf,
                siblings,
                root: self.root.clone(),
            })
        }
    }

    impl MerkleProof {
        pub fn verify(&self) -> bool {
            let mut current_hash = self.leaf.hash.clone();

            for sibling in &self.siblings {
                let mut combined = Vec::new();
                combined.extend_from_slice(b"BIL_MERKLE_NODE_V1");
                match sibling.direction {
                    MerkleDirection::Left => {
                        combined.extend_from_slice(&sibling.hash.0);
                        combined.extend_from_slice(&current_hash.0);
                    }
                    MerkleDirection::Right => {
                        combined.extend_from_slice(&current_hash.0);
                        combined.extend_from_slice(&sibling.hash.0);
                    }
                }
                current_hash = Hash256::sha256(&combined);
            }

            current_hash == self.root.0
        }
    }
}
