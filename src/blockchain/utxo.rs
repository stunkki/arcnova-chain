use std::collections::HashMap;
use crate::crypto::hash::Hash;
use super::transaction::{Transaction, TxOutput};

#[derive(Default)]
pub struct UtxoSet {
    pub utxos: HashMap<(Hash, u32), TxOutput>,
}

impl UtxoSet {
    pub fn apply_transaction(&mut self, tx: &Transaction) {
        // Remove inputs
        for input in &tx.inputs {
            self.utxos.remove(&(input.txid, input.index));
        }

        // Add outputs
        for (i, output) in tx.outputs.iter().enumerate() {
            self.utxos.insert((tx.id, i as u32), output.clone());
        }
    }

    pub fn apply_block(&mut self, block: &crate::blockchain::block::Block) {
        for tx in &block.transactions {
            self.apply_transaction(tx);
        }
    }

    pub fn find_unspent(&self, address: &str) -> Vec<((Hash, u32), TxOutput)> {
        self.utxos
            .iter()
            .filter(|(_, out)| out.address == address)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn available_balance(&self, address: &str) -> u64 {
        self.find_unspent(address)
            .iter()
            .map(|(_, out)| out.value)
            .sum()
    }
}
