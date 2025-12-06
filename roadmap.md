Phase 1 — UTXO Core Layer

Goal: Fully-validating Bitcoin-like transaction layer.

Tasks

Implement Transaction, TxInput, TxOutput

Transaction hashing

Signature verification (Ed25519)

UTXO Set (in-memory)

Apply transaction → update UTXO set

Double-spend prevention

Coinbase transaction logic

Transaction builder

Wallet key generation & addressing

Result

✔ You can send transactions
✔ State updates correctly
✔ Wallet balances are computed

Phase 2 — Blocks & Chain Logic

Goal: Single-node blockchain that mines blocks.

Tasks

Implement Block struct (header + tx list)

Block hashing

Difficulty target

Proof-of-Work (PoW) mining

Merkle root calculation

Validate blocks:

Correct PoW

Valid transactions

No double-spends

Chain data structure

Longest-chain rule

Result

✔ Blocks can be mined
✔ Invalid blocks are rejected
✔ Chain grows correctly

Phase 3 — Mempool

Goal: Nodes accept pending transactions.

Tasks

Accept transactions into mempool

Validate before adding:

Check signatures

Check UTXOs

Check fees

Remove transactions once mined

Basic fee prioritization

Result

✔ Node accepts pending transactions
✔ Miner selects from mempool

Phase 4 — P2P Networking

Goal: Multiple nodes discover each other and sync blocks.

Tasks

Peer discovery

Handshake protocol

Broadcast:

Transactions

Blocks

Request missing blocks

Sync blockchain on startup

Resolve forks using longest-chain rule

Result

✔ Network of nodes stays in sync
✔ Blocks/transactions propagate
✔ Forks resolve automatically

Phase 5 — Persistent Storage

Goal: Node survives restarts.

Tasks

RocksDB or sled integration

Store blocks by hash

Index blocks by height

Store UTXO set

Node startup procedure:

Load blocks

Rebuild UTXO set

Load mempool

Result

✔ Persistent blockchain
✔ Crash-safe node

Phase 6 — Developer Tools

Goal: Make the chain usable for devs and testers.

Tasks

CLI Wallet

Create address

Check balance

Build/send transactions

JSON-RPC API

getblock

getbalance

getmempool

sendrawtransaction

Improve transaction builder

Provide developer example scripts

Unit + integration test suites

Result

✔ Developers can interact with the chain
✔ Scripts and tooling make testing easy

Phase 7 — Test Suite

Goal: Ensure correctness, security, and stability.

Unit Tests

Transaction validation

Signature verification

UTXO selection & change handling

Fee calculation

Coinbase validation

Double-spend rejection

Integration Tests

Block mining flow

Mempool acceptance

Fork resolution (longest chain)

Merkle tree correctness

P2P block/tx sync

Restart recovery tests

Result

✔ High confidence in system reliability
✔ Safe refactoring
✔ CI-ready

Phase 8 — Smart Contract Engine (Optional)

Goal: Introduce programmability.

Tasks

Define a tiny bytecode VM

Create instruction set (math, branching, storage)

Contract storage model

Gas metering

Contract deployment

Contract execution inside UTXO context

Result

✔ Programmable blockchain
✔ Smart contracts similar to Bitcoin Script / Miniscript

Phase 9 — HTTP API + Explorer (Optional)

Goal: Make ArcNova user-friendly.

Tasks

HTTP JSON endpoints

Simple block explorer UI

Transaction search

Address balance lookup

Result

✔ User-friendly blockchain
✔ Testnet-ready explorer