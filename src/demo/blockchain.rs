use crate::block::Block;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use itoa;
use ed25519_dalek::VerifyingKey;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        let difficulty = 4; // adjust as needed, could be even env var 

        let genesis = Block::new(
            0,
            vec![],   // no transactions
            "0".into(),
            difficulty
        );

        Self {
            chain: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, txs: Vec<Transaction>) {
        let last = self.chain.last().unwrap();

        let block = Block::new(
            last.index + 1,
            txs,
            last.hash.clone(),
            self.difficulty,
        );

        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Hash correct
            if current.hash != current.calculate_hash() {
                return false;
            }

            // Matches difficulty
            let target = "0".repeat(current.difficulty as usize);
            if !current.hash.starts_with(&target) {
                return false;
            }

            // Linked correctly
            if current.prev_hash != previous.hash {
                return false;
            }

            // Verify transaction signatures
            for tx in &current.transactions {
                // For demo: someone could map addresses to public keys
                // Here assume signature exists 
                if tx.signature.is_none() {
                    return false;
                }
            }
        }
        true
    }
}
