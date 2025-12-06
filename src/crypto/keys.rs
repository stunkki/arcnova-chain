use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use sha2::{Digest, Sha512};

pub fn sign_message(keypair: &Keypair, msg: &[u8]) -> Vec<u8> {
    // ed25519 internally hashes with SHA-512
    let sig: Signature = keypair.sign(msg);
    sig.to_bytes().to_vec()
}

pub fn verify_signature(public_key_bytes: &[u8], sig_bytes: &[u8], msg: &[u8]) -> bool {
    let public_key = match PublicKey::from_bytes(public_key_bytes) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    let sig = match Signature::from_bytes(sig_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };

    public_key.verify(msg, &sig).is_ok()
}