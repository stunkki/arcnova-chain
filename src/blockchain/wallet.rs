use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);
        Wallet { keypair }
    }

    pub fn address(&self) -> String {
        let hash = Sha256::digest(self.keypair.public.as_bytes());
        // simple "pubkey hash" address, first 20 bytes
        hex::encode(&hash[0..20])
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
}