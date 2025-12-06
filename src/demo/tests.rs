// src/tests.rs
use crate::p2p; 
use libp2p::futures::StreamExt;
use std::time::Duration;

pub fn print_build_info() {
    println!("ArcNova Chain Build Info:");
    println!(" ├ Git SHA: {}", GIT_SHA);
    println!(" └ Built:   {}", BUILD_TIME);
}

use crate::{
    blockchain_core::{blockchain::Blockchain, transaction::Transaction, wallet::Wallet},
    // If print_build_info is moved to a top-level util module, import it here
};

#[test]
fn test_core_blockchain_flow() {
    // 1. Setup Wallets
    let alice = Wallet::new();
    let bob = Wallet::new();

    // 2. Initialize Chain
    let mut chain = Blockchain::new();

    // 3. Create and Sign Transaction
    let mut tx = Transaction::new(alice.address.clone(), bob.address.clone(), 100);
    tx.sign(&alice.private_key);

    // 4. Mining
    // In a test, you typically want fast mining, not real PoW time.
    // If add_block performs PoW, this will take time, but it ensures the core logic works.
    println!("Mining block...");
    chain.add_block(vec![tx]);

    // 5. Verification
    println!("Chain state:\n{:#?}", chain);
    assert!(chain.is_valid(), "The newly mined chain should be valid.");
}

// Test peer discovery and message gossip
#[tokio::test] 
async fn test_p2p_node_discovery_and_gossip() {
    // 1. Initialize Node 1
    let mut swarm_1 = p2p::start_p2p_node()
        .await
        .expect("Failed node 1");
    
    // Wait for Node 1 to listen
    let listen_addr_1 = swarm_1
        .select_next_some()
        .await
        .next_listen_addr()
        .expect("Node 1 no listen addr");

    // 2. Initialize Node 2
    let mut swarm_2 = p2p::start_p2p_node()
        .await
        .expect("Failed node 2");

    // Node 2 dials Node 1
    let peer_id_1 = *swarm_1.local_peer_id();
    swarm_2
        .dial(listen_addr_1)
        .expect("Node 2 failed to dial");

    // 3. Gossip a message from Node 2
    let test_message = b"TEST_GOSSIP_MESSAGE";
    swarm_2.behaviour_mut().floodsub.publish(p2p::block_topic(), test_message.to_vec());
    
    // 4. Node 1 waits for the message
    let mut received = false;
    let timeout = tokio::time::sleep(Duration::from_secs(5));
    
    tokio::select! {
        event = swarm_1.select_next_some() => {
            if let libp2p::swarm::SwarmEvent::Behaviour(p2p::AppBehaviourEvent::Floodsub(
                p2p::FloodsubEvent::Message(msg)
            )) = event {
                if msg.data == test_message {
                    received = true;
                }
            }
        }
        _ = timeout => {} 
    }

    // 5. Assertion
    assert!(received, "Node 1 did not receive the message.");
}