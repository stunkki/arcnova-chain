Need to think if we go with Bitcoin-like or Ethereum-like

1. UTXO Model (Bitcoin-like)

- Implement transaction inputs and outputs
- Prevent double-spending
- Track UTXO set for wallets

2. Account State (Ethereum-like)

- Track balance per address
- State transition logic
- Nonces for replay protection

3. Peer-to-Peer Networking

- Discover peers
- Broadcast new blocks
- Sync longest chain
- Gossip propagation

4. Persistent Storage

- Store blocks in RocksDB
- Maintain index of block heights
- Recover blockchain at startup

5. Smart Contract Engine

- Simple bytecode VM
- Opcodes for math, storage, branching
- Gas metering

Contract deployment & execution

6. Dev Tools

- CLI wallet for key management
- Transaction builder
- Explorer JSON-RPC
- Unit/integration tests