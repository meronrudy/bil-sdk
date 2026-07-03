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

impl BilValue {
    pub fn normalize_timestamp(ms: i64) -> Self {
        // Timestamps are normalized to integer milliseconds
        BilValue::Integer(ms as i128)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash256(pub [u8; 32]);

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};

impl Hash256 {
    pub fn zero() -> Self {
        Hash256([0; 32])
    }

    pub fn sha256(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Hash256(hash)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    pub fn from_hex(s: &str) -> Result<Self, CanonicalError> {
        let bytes = hex::decode(s).map_err(|e| CanonicalError::Encoding(e.to_string()))?;
        if bytes.len() != 32 {
            return Err(CanonicalError::Encoding("Invalid hash length".to_string()));
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes);
        Ok(Hash256(hash))
    }

    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(self.0)
    }

    pub fn from_base64(s: &str) -> Result<Self, CanonicalError> {
        let bytes = BASE64_STANDARD.decode(s).map_err(|e| CanonicalError::Encoding(e.to_string()))?;
        if bytes.len() != 32 {
            return Err(CanonicalError::Encoding("Invalid hash length".to_string()));
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes);
        Ok(Hash256(hash))
    }
}

impl Default for Hash256 {
    fn default() -> Self {
        Self::zero()
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

impl TryFrom<&serde_json::Value> for BilValue {
    type Error = CanonicalError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Null => Ok(BilValue::Null),
            serde_json::Value::Bool(b) => Ok(BilValue::Bool(*b)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(BilValue::Integer(i as i128))
                } else if let Some(u) = n.as_u64() {
                    Ok(BilValue::Integer(u as i128))
                } else {
                    Err(CanonicalError::Encoding("Floats are not supported in canonical BIL".to_string()))
                }
            }
            serde_json::Value::String(s) => Ok(BilValue::Text(s.clone())),
            serde_json::Value::Array(arr) => {
                let mut bil_arr = Vec::with_capacity(arr.len());
                for item in arr {
                    bil_arr.push(BilValue::try_from(item)?);
                }
                Ok(BilValue::Array(bil_arr))
            }
            serde_json::Value::Object(obj) => {
                let mut bil_map = Vec::with_capacity(obj.len());
                for (k, v) in obj {
                    bil_map.push((BilValue::Text(k.clone()), BilValue::try_from(v)?));
                }
                Ok(BilValue::Map(bil_map))
            }
        }
    }
}

pub fn encode_canonical(value: &BilValue) -> Result<Vec<u8>, CanonicalError> {
    let cbor_value = to_ciborium_value(value)?;
    let mut bytes = Vec::new();
    ciborium::into_writer(&cbor_value, &mut bytes)
        .map_err(|e| CanonicalError::Encoding(e.to_string()))?;
    Ok(bytes)
}

fn to_ciborium_value(value: &BilValue) -> Result<ciborium::Value, CanonicalError> {
    match value {
        BilValue::Null => Ok(ciborium::Value::Null),
        BilValue::Bool(b) => Ok(ciborium::Value::Bool(*b)),
        BilValue::Integer(i) => {
            if let Ok(v) = i64::try_from(*i) {
                Ok(ciborium::Value::Integer(v.into()))
            } else {
                // ciborium::Value::Integer only supports up to i128 via TryFrom, but internally it's i128.
                // Let's just use TryInto directly.
                Ok(ciborium::Value::Integer((*i).try_into().map_err(|_| CanonicalError::Encoding("Integer too large for CBOR".to_string()))?))
            }
        }
        BilValue::Bytes(b) => Ok(ciborium::Value::Bytes(b.clone())),
        BilValue::Text(t) => Ok(ciborium::Value::Text(t.clone())),
        BilValue::Array(arr) => {
            let mut cbor_arr = Vec::with_capacity(arr.len());
            for item in arr {
                cbor_arr.push(to_ciborium_value(item)?);
            }
            Ok(ciborium::Value::Array(cbor_arr))
        }
        BilValue::Map(map) => {
            let mut cbor_map = Vec::with_capacity(map.len());
            for (k, v) in map {
                cbor_map.push((to_ciborium_value(k)?, to_ciborium_value(v)?));
            }
            // Deterministic CBOR requires keys to be sorted
            cbor_map.sort_by(|a, b| {
                let mut a_bytes = Vec::new();
                let mut b_bytes = Vec::new();
                ciborium::into_writer(&a.0, &mut a_bytes).unwrap();
                ciborium::into_writer(&b.0, &mut b_bytes).unwrap();
                a_bytes.cmp(&b_bytes)
            });
            Ok(ciborium::Value::Map(cbor_map))
        }
    }
}
