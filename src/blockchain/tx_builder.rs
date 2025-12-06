use thiserror::Error;
use crate::blockchain::transaction::{Transaction, TxInput, TxOutput};
use crate::blockchain::utxo::UtxoSet;
use crate::blockchain::state::State;
use crate::blockchain::wallet::Wallet;
use crate::crypto::hash::Hash;
use crate::crypto::keys::sign_message;

#[derive(Debug, Error)]
pub enum TxBuildError {
    #[error("insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },

    #[error("no UTXOs available for this address")]
    NoUtxos,

    #[error("internal error: {0}")]
    Internal(String),
}

pub struct TxBuilder<'a> {
    pub state: &'a State,
}

impl<'a> TxBuilder<'a> {
    pub fn new(state: &'a State) -> Self {
        TxBuilder { state }
    }

    /// Create a standard payment transaction:
    /// - from `wallet`
    /// - to `to_address`
    /// - amount `amount`
    /// - with fee `fee`
    pub fn build_payment_tx(
        &self,
        wallet: &Wallet,
        to_address: &str,
        amount: u64,
        fee: u64,
    ) -> Result<Transaction, TxBuildError> {
        let from_address = wallet.address();
        let utxos = self.state.utxo_set.find_unspent(&from_address);

        if utxos.is_empty() {
            return Err(TxBuildError::NoUtxos);
        }

        let target = amount + fee;

        // 1. Select UTXOs (simple greedy strategy)
        let mut selected = Vec::new();
        let mut total_in = 0u64;

        for (key, out) in utxos {
            selected.push((key, out.clone()));
            total_in += out.value;

            if total_in >= target {
                break;
            }
        }

        if total_in < target {
            return Err(TxBuildError::InsufficientFunds {
                required: target,
                available: total_in,
            });
        }

        // 2. Build inputs
        let public_key_bytes = wallet.public_key_bytes();
        let mut inputs = Vec::new();

        for ((txid, index), _output) in &selected {
            inputs.push(TxInput {
                txid: txid.clone(),
                index: *index,
                signature: Vec::new(),     // filled in later
                public_key: public_key_bytes.clone(),
            });
        }

        // 3. Build outputs (recipient + change)
        let mut outputs = Vec::new();

        // recipient
        outputs.push(TxOutput {
            value: amount,
            address: to_address.to_string(),
        });

        // change (if any)
        let change = total_in - target;
        if change > 0 {
            outputs.push(TxOutput {
                value: change,
                address: from_address,
            });
        }

        // 4. Build unsigned transaction
        let mut tx = Transaction {
            id: Hash([0u8; 32]),
            inputs,
            outputs,
        };

        // 5. Compute tx id (hash of tx skeleton)
        let txid = tx.calculate_hash();
        tx.id = txid.clone();

        // 6. Sign each input with the wallet key
        //    (Sign over the txid; could also sign serialized tx data)
        for input in tx.inputs.iter_mut() {
            let sig = sign_message(&wallet.keypair, &tx.id.0);
            input.signature = sig;
        }

        Ok(tx)
    }
}