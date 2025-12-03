// src/p2p/mod.rs
use libp2p::{
    core::upgrade::Version,
    futures::StreamExt,
    identity, 
    PeerId, Swarm, 
    swarm::{NetworkBehaviour, SwarmEvent},
    identify, mdns, 
    floodsub::{Floodsub, FloodsubEvent, Topic},
};
use tokio::io;

// Gossip topic constants
pub const BLOCK_TOPIC_STR: &str = "arcnova-blocks";
pub const TX_TOPIC_STR: &str = "arcnova-transactions";

// Helper functions for Topic creation
pub fn block_topic() -> Topic { Topic::new(BLOCK_TOPIC_STR) }
pub fn tx_topic() -> Topic { Topic::new(TX_TOPIC_STR) }

// Custom Network Behavior combining protocols
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "AppBehaviourEvent")]
pub struct AppBehaviour {
    mdns: mdns::tokio::Behaviour,
    floodsub: Floodsub, 
    identify: identify::Behaviour,
}

// Unified event enum
#[derive(Debug)]
pub enum AppBehaviourEvent {
    Floodsub(FloodsubEvent),
    Mdns(mdns::Event),
    Identify(identify::Event),
}

// Event conversion implementations
impl From<FloodsubEvent> for AppBehaviourEvent {
    fn from(event: FloodsubEvent) -> Self { AppBehaviourEvent::Floodsub(event) }
}
impl From<mdns::Event> for AppBehaviourEvent {
    fn from(event: mdns::Event) -> Self { AppBehaviourEvent::Mdns(event) }
}
impl From<identify::Event> for AppBehaviourEvent {
    fn from(event: identify::Event) -> Self { AppBehaviourEvent::Identify(event) }
}

// Network initialization
pub async fn start_p2p_node() -> Result<Swarm<AppBehaviour>, Box<dyn std::error::Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    
    let transport = libp2p::development_transport(local_key.clone()).await
        .expect("Failed transport");

    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)
        .expect("Failed mDNS");
    
    let mut floodsub = Floodsub::new(local_peer_id);
    
    floodsub.subscribe(block_topic());
    floodsub.subscribe(tx_topic());

    let identify = identify::Behaviour::new(
        identify::Config::new("/arcnova/1.0.0".to_owned(), local_key.public())
            .with_protocol_version(Version::V1),
    );

    let mut swarm = Swarm::new(
        transport, 
        AppBehaviour { mdns, floodsub, identify }, 
        local_peer_id
    );

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("Local Peer ID: {}", local_peer_id);
    
    Ok(swarm)
}

// Main network event loop
pub async fn run_p2p_event_loop(mut swarm: Swarm<AppBehaviour>) {
    loop {
        tokio::select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on: {:?}", address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    println!("Connection established with: {}", peer_id);
                }
                SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, addr) in list {
                        swarm.behaviour_mut().floodsub.add_node_to_partial_view(peer_id);
                    }
                }
                SwarmEvent::Behaviour(AppBehaviourEvent::Floodsub(FloodsubEvent::Message(msg))) => {
                    if msg.topic.to_string() == BLOCK_TOPIC_STR {
                        // TODO: Deserialize and validate block
                        println!("Received new BLOCK ({} bytes)", msg.data.len());
                    } else if msg.topic.to_string() == TX_TOPIC_STR {
                        // TODO: Deserialize and add transaction to mempool
                        println!("Received new TRANSACTION ({} bytes)", msg.data.len());
                    }
                }
                _ => {}
            },
        }
    }
}