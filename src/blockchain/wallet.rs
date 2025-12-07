use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

pub struct Wallet {
    pub keypair: Keypair,
}

pub fn address_from_public_key_bytes(public_key_bytes: &[u8]) -> Option<String> {
    if public_key_bytes.len() != 32 {
        return None;
    }

    let hash = Sha256::digest(public_key_bytes);
    Some(hex::encode(&hash[0..20]))
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);
        Wallet { keypair }
    }

    pub fn address(&self) -> String {
        address_from_public_key_bytes(self.keypair.public.as_bytes())
            .expect("public key length should always be valid")
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
}