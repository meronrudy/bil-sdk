use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CanonicalError {
    #[error("Encoding error: {0}")]
    Encoding(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BilValue {
    Null,
    Bool(bool),
    Integer(i128),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<BilValue>),
    Map(Vec<(BilValue, BilValue)>),
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash256(pub [u8; 32]);

impl Hash256 {
    pub fn sha256(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Hash256(hash)
    }
}

pub trait BilCanonical {
    fn to_canonical_value(&self) -> Result<BilValue, CanonicalError>;

    fn to_canonical_bytes(&self) -> Result<Vec<u8>, CanonicalError> {
        encode_canonical(&self.to_canonical_value()?)
    }

    fn commitment_hash(&self) -> Result<Hash256, CanonicalError> {
        Ok(Hash256::sha256(&self.to_canonical_bytes()?))
    }
}

pub fn encode_canonical(_value: &BilValue) -> Result<Vec<u8>, CanonicalError> {
    // Placeholder for dCBOR encoding
    Ok(vec![])
}
