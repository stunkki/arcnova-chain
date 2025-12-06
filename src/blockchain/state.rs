use super::{transaction::Transaction, utxo::UtxoSet};
use crate::crypto::keys::verify_signature;

pub struct State {
    pub utxo_set: UtxoSet,
}

impl State {
    pub fn validate_transaction(&self, tx: &Transaction) -> bool {
        // Coinbase always valid
        if tx.is_coinbase() {
            return true;
        }

        let mut input_sum = 0;
        let mut output_sum = 0;

        // 1. Validate signatures + ownership
        for input in &tx.inputs {
            let utxo = match self.utxo_set.utxos.get(&(input.txid, input.index)) {
                Some(u) => u,
                None => return false,
            };

            if !verify_signature(&input.public_key, &input.signature, &tx.id.0) {
                return false;
            }

            // 2. Sum inputs
            input_sum += utxo.value;
        }

        // 3. Sum outputs
        for o in &tx.outputs {
            output_sum += o.value;
        }

        // 4. Prevent inflation
        input_sum >= output_sum
    }
}