use serde::{Serialize, Deserialize};
use crate::crypto::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    pub txid: Hash,      // referenced transaction
    pub index: u32,      // which UTXO is spent
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutput {
    pub value: u64,      // amount
    pub address: String, // receiver address (pubkey hash)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Hash,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn calculate_hash(&self) -> Hash {
        Hash::hash(self)
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
}