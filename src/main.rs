include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

use blockchain_core::{blockchain::Blockchain, transaction::Transaction, wallet::Wallet};


pub fn print_build_info() {
    println!("ArcNova Chain Build Info:");
    println!(" ├ Git SHA: {}", GIT_SHA);
    println!(" └ Built:   {}", BUILD_TIME);
}

fn main() {
    print_build_info();

    // Example usage of your blockchain
    let alice = Wallet::new();
    let bob = Wallet::new();

    let mut chain = Blockchain::new();

    let mut tx = Transaction::new(alice.address.clone(), bob.address.clone(), 100);
    tx.sign(&alice.private_key);

    println!("Mining block...");
    chain.add_block(vec![tx]);

    println!("{:#?}", chain);
    println!("Valid? {}", chain.is_valid());
}