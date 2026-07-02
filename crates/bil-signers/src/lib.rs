use bil_core::AssuranceLevel;
use bil_ink::{SignatureBytes, SignerRef};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("Signing failed: {0}")]
    Failed(String),
}

#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("Verification failed: {0}")]
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct PublicKeyRef(pub String);

#[derive(Debug, Clone)]
pub enum SignatureAlgorithm {
    Ed25519,
    EcdsaP256,
}

pub trait BilSigner: Send + Sync {
    fn signer_id(&self) -> SignerRef;
    fn algorithm(&self) -> SignatureAlgorithm;
    fn assurance_level(&self) -> AssuranceLevel;
    fn public_key_ref(&self) -> PublicKeyRef;

    fn sign(&self, canonical_bytes: &[u8]) -> Result<SignatureBytes, SignerError>;
}

pub trait BilSignatureVerifier {
    fn verify_signature(
        &self,
        public_key: &PublicKeyRef,
        canonical_bytes: &[u8],
        signature: &SignatureBytes,
    ) -> Result<(), SignatureError>;
}
