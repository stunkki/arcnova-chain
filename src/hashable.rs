use sha2::{Sha256, Digest};
use std::io::Write;

pub type Hash = Vec<u8>;

pub trait Hashable {
    // Defines the canonical byte representation of the structure for hashing.
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(self.bytes());
        hasher.finalize().to_vec()
    }
}

/*
use crate::hashable::{Hash, Hashable}; // Assuming the new trait is in src/hashable.rs
use std::io::Write;

pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub merkle_root: Hash,
    pub prev_hash: Hash,
    pub nonce: u64,
    pub difficulty: u32,
    // ... other fields (e.g., transactions)
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        
        // Use little-endian byte order for cross-platform hash consistency
        buffer.write_all(&self.index.to_le_bytes()).unwrap();
        buffer.write_all(&self.timestamp.to_le_bytes()).unwrap();
        buffer.write_all(&self.nonce.to_le_bytes()).unwrap();
        buffer.write_all(&self.difficulty.to_le_bytes()).unwrap();
        
        // Merkle root and previous hash are already bytes
        buffer.write_all(&self.merkle_root).unwrap();
        buffer.write_all(&self.prev_hash).unwrap();
        
        buffer
    }
}
*/