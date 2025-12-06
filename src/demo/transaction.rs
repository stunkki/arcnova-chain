use serde::{Serialize, Deserialize};
use ed25519_dalek::{Signature, VerifyingKey};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String,       // address
    pub to: String,         // address
    pub amount: u64,
    pub signature: Option<Vec<u8>>,   // signature bytes
}

impl Transaction {
    /// Create an unsigned transaction
    pub fn new(from: String, to: String, amount: u64) -> Self {
        Transaction {
            from,
            to,
            amount,
            signature: None,
        }
    }

    /// Serialize fields for signing
    fn bytes_for_signing(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.from);
        hasher.update(&self.to);
        hasher.update(self.amount.to_be_bytes());
        hasher.finalize().to_vec()
    }

    /// Sign using a private key from a wallet
    pub fn sign(&mut self, signing_key: &ed25519_dalek::SigningKey) {
        let message = self.bytes_for_signing();
        let sig = signing_key.sign(&message);
        self.signature = Some(sig.to_bytes().to_vec());
    }

    /// Verify signature using the sender's public key
    pub fn verify(&self, public_key: &VerifyingKey) -> bool {
        if let Some(sig_bytes) = &self.signature {
            if let Ok(sig) = Signature::from_bytes(sig_bytes) {
                let message = self.bytes_for_signing();
                return public_key.verify_strict(&message, &sig).is_ok();
            }
        }
        false
    }
}
