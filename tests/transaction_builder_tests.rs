use arcnova_chain::blockchain::{
    state::State,
    tx_builder::TxBuilder,
    transaction::{TxOutput, Transaction},
    utxo::UtxoSet,
    wallet::Wallet,
};
use arcnova_chain::crypto::hash::Hash;

// TEST 1 — UTXO selection + change logic
#[test]
fn test_tx_builder_basic_send() {
    let wallet = Wallet::new();
    let from = wallet.address();

    let fake_txid = Hash([9u8; 32]);

    let mut utxo_set = UtxoSet::default();
    utxo_set.utxos.insert(
        (fake_txid.clone(), 0),
        TxOutput {
            value: 100_000,
            address: from.clone(),
        },
    );

    let state = State { utxo_set };
    let builder = TxBuilder::new(&state);

    let tx = builder
        .build_payment_tx(&wallet, "bob", 50_000, 1_000)
        .unwrap();

    assert_eq!(tx.outputs.len(), 2);
    assert_eq!(tx.outputs[0].value, 50_000);
    assert_eq!(tx.outputs[1].value, 49_000);
    assert_eq!(tx.outputs[0].address, "bob");
    assert_eq!(tx.outputs[1].address, from);

    assert!(state.validate_transaction(&tx));
}

// TEST 2 — Insufficient funds error
#[test]
fn test_tx_builder_insufficient_funds() {
    let wallet = Wallet::new();
    let from = wallet.address();

    let fake_txid = Hash([2u8; 32]);

    let mut utxo_set = UtxoSet::default();
    utxo_set.utxos.insert(
        (fake_txid.clone(), 0),
        TxOutput {
            value: 10_000,
            address: from.clone(),
        },
    );

    let state = State { utxo_set };
    let builder = TxBuilder::new(&state);

    let err = builder
        .build_payment_tx(&wallet, "alice", 50_000, 1_000)
        .unwrap_err();

    match err {
        arcnova_chain::blockchain::tx_builder::TxBuildError::InsufficientFunds { required, available } => {
            assert_eq!(required, 51_000);
            assert_eq!(available, 10_000);
        }
        _ => panic!("wrong error variant"),
    }
}

// TEST 3 — No UTXOs available
#[test]
fn test_tx_builder_no_utxos() {
    let wallet = Wallet::new();
    let state = State { utxo_set: UtxoSet::default() };
    let builder = TxBuilder::new(&state);

    let err = builder
        .build_payment_tx(&wallet, "bob", 10_000, 100)
        .unwrap_err();

    match err {
        arcnova_chain::blockchain::tx_builder::TxBuildError::NoUtxos => {}
        _ => panic!("wrong error variant"),
    }
}

// TEST 4 — Check txid hashing correctness (not all zeros)
#[test]
fn test_txid_not_zero() {
    let wallet = Wallet::new();
    let from = wallet.address();
    let fake_txid = Hash([7u8; 32]);

    let mut utxo_set = UtxoSet::default();
    utxo_set.utxos.insert(
        (fake_txid.clone(), 0),
        TxOutput {
            value: 42_000,
            address: from.clone(),
        },
    );

    let state = State { utxo_set };
    let builder = TxBuilder::new(&state);

    let tx = builder
        .build_payment_tx(&wallet, "test", 20_000, 1_000)
        .unwrap();

    assert_ne!(tx.id.0, [0u8; 32]);
}

/// TEST 5 — Validate transaction signatures
#[test]
fn test_transaction_signature_valid() {
    let wallet = Wallet::new();
    let from = wallet.address();

    let fake_txid = Hash([3u8; 32]);

    let mut utxo_set = UtxoSet::default();
    utxo_set.utxos.insert(
        (fake_txid.clone(), 0),
        TxOutput {
            value: 60_000,
            address: from.clone(),
        },
    );

    let state = State { utxo_set };
    let builder = TxBuilder::new(&state);

    let tx = builder
        .build_payment_tx(&wallet, "bob", 10_000, 1_000)
        .unwrap();

    assert!(state.validate_transaction(&tx));
}

/// TEST 6 — Invalid signature should fail
#[test]
fn test_invalid_signature_fails_validation() {
    let wallet = Wallet::new();
    let from = wallet.address();

    let fake_txid = Hash([5u8; 32]);

    let mut utxo_set = UtxoSet::default();
    utxo_set.utxos.insert(
        (fake_txid.clone(), 0),
        TxOutput {
            value: 80_000,
            address: from.clone(),
        },
    );

    let state = State { utxo_set };
    let builder = TxBuilder::new(&state);

    let mut tx = builder
        .build_payment_tx(&wallet, "bob", 20_000, 1_000)
        .unwrap();

    // Corrupt signature to force invalid
    tx.inputs[0].signature = vec![9, 9, 9, 9];

    assert!(!state.validate_transaction(&tx));
}

/// TEST 7 — Double-spend detection
#[test]
fn test_double_spend_detected() {
    let wallet = Wallet::new();
    let from = wallet.address();
    let fake_txid = Hash([8u8; 32]);

    let mut utxo_set = UtxoSet::default();

    utxo_set.utxos.insert(
        (fake_txid.clone(), 0),
        TxOutput {
            value: 100_000,
            address: from.clone(),
        },
    );

    // Build initial state
    let mut state = State { utxo_set };

    // Build first tx
    let builder = TxBuilder::new(&state);
    let tx1 = builder.build_payment_tx(&wallet, "bob", 30_000, 1_000).unwrap();
    assert!(state.validate_transaction(&tx1));

    // Apply first tx (spends the UTXO)
    state.utxo_set.apply_transaction(&tx1);

    // Build second tx which attempts to spend same UTXO again
    let builder2 = TxBuilder::new(&state);
    let tx2 = builder2.build_payment_tx(&wallet, "alice", 20_000, 1_000);

    // This should fail (no UTXOs left)
    assert!(tx2.is_err());
}

/// TEST 8 — Coinbase transaction is always valid
#[test]
fn test_coinbase_is_valid() {
    let coinbase = Transaction {
        id: Hash([1u8; 32]),
        inputs: vec![],   // coinbase has no inputs
        outputs: vec![TxOutput {
            value: 50_000,
            address: "miner1".into(),
        }],
    };

    let state = State {
        utxo_set: UtxoSet::default(),
    };

    assert!(state.validate_transaction(&coinbase));
}



//
// TEST 9 — Sum(inputs) < sum(outputs) must be invalid (inflation check)
//
#[test]
fn test_invalid_inflation_transaction() {
    let wallet = Wallet::new();
    let from = wallet.address();
    let fake_txid = Hash([4u8; 32]);

    let mut utxo_set = UtxoSet::default();
    utxo_set.utxos.insert(
        (fake_txid.clone(), 0),
        TxOutput {
            value: 20_000,
            address: from.clone(),
        },
    );

    let state = State { utxo_set };
    let builder = TxBuilder::new(&state);

    // Force an invalid tx with too-large output
    let mut tx = builder
        .build_payment_tx(&wallet, "bob", 19_000, 1_000)
        .unwrap();

    // Modify outputs to exceed total inputs
    tx.outputs[0].value = 50_000;

    assert!(!state.validate_transaction(&tx));
}