include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

use blockchain_core::{blockchain::Blockchain, transaction::Transaction, wallet::Wallet};

// p2p module
// src/main.rs

// Include P2P module
mod p2p; 
// Include test module only when running tests
#[cfg(test)] 
mod tests; 

// Enable the asynchronous runtime
#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ArcNova Chain Node Starting...");

    // TODO: Initialize chain state

    // Initialize P2P Swarm
    let swarm = p2p::start_p2p_node().await?;

    // Run the P2P event loop concurrently
    let network_handle = tokio::spawn(p2p::run_p2p_event_loop(swarm));

    println!("⛏️ Starting main application loop...");
    
    loop {
        // Main application logic (Mining, CLI, etc.)
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
    
    // network_handle.await.unwrap();
    // Ok(())
}