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

pub struct SoftwareDevSigner {
    signer_id: SignerRef,
    keypair: ed25519_dalek::SigningKey,
}

impl SoftwareDevSigner {
    pub fn new(signer_id: String) -> Self {
        use rand::rngs::OsRng;
        let mut csprng = OsRng;
        let keypair = ed25519_dalek::SigningKey::generate(&mut csprng);
        Self {
            signer_id: SignerRef(signer_id),
            keypair,
        }
    }
}

impl BilSigner for SoftwareDevSigner {
    fn signer_id(&self) -> SignerRef {
        self.signer_id.clone()
    }

    fn algorithm(&self) -> SignatureAlgorithm {
        SignatureAlgorithm::Ed25519
    }

    fn assurance_level(&self) -> AssuranceLevel {
        AssuranceLevel::L0SoftwareDev
    }

    fn public_key_ref(&self) -> PublicKeyRef {
        // In a real implementation, this would be a proper reference (e.g., a DID or JWK)
        // For now, we just hex-encode the public key bytes
        let pk_bytes = self.keypair.verifying_key().to_bytes();
        PublicKeyRef(hex::encode(pk_bytes))
    }

    fn sign(&self, canonical_bytes: &[u8]) -> Result<SignatureBytes, SignerError> {
        use ed25519_dalek::Signer;
        let signature = self.keypair.sign(canonical_bytes);
        Ok(SignatureBytes(signature.to_bytes().to_vec()))
    }
}
