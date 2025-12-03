// current purpose is only for quick testing/prototyping
// using transactions + merkle trees
//

/*
use blockchain_core::{
    blockchain::Blockchain,
    transaction::Transaction,
    wallet::Wallet,
};

fn main() {
    // Create wallets
    let alice = Wallet::new();
    let bob = Wallet::new();

    println!("Alice address: {}", alice.address);
    println!("Bob address:   {}", bob.address);

    let mut chain = Blockchain::new();

    // Create a transaction
    let mut tx = Transaction::new(alice.address.clone(), bob.address.clone(), 100);

    // Alice signs the transaction
    tx.sign(&alice.private_key);

    // Add block with signed transaction
    println!("Mining block...");
    chain.add_block(vec![tx]);

    println!("{:#?}", chain);
    println!("Valid? {}", chain.is_valid());
}

*/
