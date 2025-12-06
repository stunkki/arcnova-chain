use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use crate::transaction::Transaction;
use crate::merkle::merkle_root;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u32,
}

impl Block {
    pub fn new(
        index: u64, 
        transactions: Vec<Transaction>, 
        prev_hash: String, 
        difficulty: u32
    ) -> Self {

        let timestamp = chrono::Utc::now().timestamp_millis() as u128;

        let merkle_root = merkle_root(
            transactions.iter().map(|t| serde_json::to_string(t).unwrap()).collect()
        );

        let mut block = Block {
            index,
            timestamp,
            transactions,
            merkle_root,
            prev_hash,
            hash: String::new(),
            nonce: 0,
            difficulty,
        };

        block.mine();
        block
    }

    /// Hash function (includes nonce + difficulty)
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.merkle_root);
        hasher.update(&self.prev_hash);
        hasher.update(self.nonce.to_string());
        hasher.update(self.difficulty.to_string());
        format!("{:x}", hasher.finalize())
    }

    /// Proof-of-Work mining loop
    pub fn mine(&mut self) {
        let target = "0".repeat(self.difficulty as usize);

        loop {
            let hash = self.calculate_hash();
            if hash.starts_with(&target) {
                self.hash = hash;
                break;
            }
            self.nonce += 1;
        }
    }
}
