# Architecture Overview

This document describes the internal architecture of the **Blockchain Core (Rust)** project.  
The design emphasizes simplicity, readability, and correctness as a learning-focused blockchain implementation.

---

# 1. High-Level System Diagram

+------------------------+
| Wallet |
| - Keypair (Ed25519) |
| - Address creation |
| - Sign messages |
+-----------+------------+
|
v
+------------------------+
| Transaction |
| - from/to |
| - amount |
| - signature (optional) |
| - hashing for signing |
+-----------+------------+
|
v
+------------------------+
| Merkle Tree |
| - Hash transaction |
| - Build tree layers |
| - Produce merkle root |
+-----------+------------+
|
v
+---------------------------------------+
| Block |
| - index |
| - timestamp |
| - prev_hash |
| - transactions[] |
| - merkle root |
| - nonce |
| - difficulty |
| - hash |
| - mining (Proof-of-Work) |
+------------------+--------------------+
|
v
+---------------------------------------+
| Blockchain |
| - Vector of blocks |
| - Add block with PoW |
| - Validate full chain |
| - Verify signatures |
| - Check linkage & difficulty |
+---------------------------------------+

yaml
Copy code

---

# 2. Modules

## 2.1 `wallet.rs`
Handles keypair generation and signing.

- **Ed25519 keypair** using `ed25519-dalek`
- Public key → address via:
  - SHA-256 hash
  - First 20 bytes as the address
- Message signing for transactions

This module never stores private keys on disk — it is stateless.

---

## 2.2 `transaction.rs`
Represents a signed or unsigned value transfer.

- Contains:
  - `from` address
  - `to` address
  - `amount`
  - optional `signature`
- Transactions must be signed by the sender
- Verification uses the sender’s public key

Serialization is deterministic to ensure signing is reproducible.

---

## 2.3 `merkle.rs`
Builds Merkle Trees for transaction integrity.

- Leaf hash = SHA-256 of transaction JSON
- Internal nodes hash left+right concatenation
- Supports odd leaf duplication
- Returns a **single merkle root**

This root is stored in each block.

---

## 2.4 `block.rs`
Defines and mines a block.

Fields:

- `index`
- `timestamp`
- `prev_hash`
- `transactions`
- `merkle_root`
- `nonce`
- `difficulty`
- `hash`

Proof-of-Work:

- Difficulty expressed as number of leading zeroes
- Nonce increments until `hash < target`
- Hash includes:
  - index
  - timestamp
  - merkle_root
  - prev_hash
  - nonce
  - difficulty

---

## 2.5 `blockchain.rs`
Manages the chain of blocks.

Responsibilities:

- Initialize genesis block  
- Add new blocks (mining required)  
- Validate:
  - Transaction signatures  
  - Hash consistency  
  - PoW difficulty  
  - Previous hash linking  
  - Merkle root matching  
- Enforce immutability of block history  

The chain is stored in memory as a `Vec<Block>`.

---

# 3. Data Flow (End-to-End)

Wallet → Transaction → Block → Blockchain → Validation

yaml
Copy code

1. A wallet signs a transaction  
2. The transaction is included in a block  
3. Block builds a Merkle tree  
4. Block is mined using Proof-of-Work  
5. Blockchain verifies:  
   - block hash  
   - linkage  
   - PoW difficulty  
   - transaction signatures  

---

# 4. Design Goals

### ✔ Educational clarity  
Every component is implemented with maximum transparency.

### ✔ Modular architecture  
Each module is independent and composable.

### ✔ Safety-first Rust  
Ownership, type guarantees, and secure cryptography.

### ✔ Extensible  
Future roadmap includes:
- UTXO model  
- Account state  
- P2P networking  
- Smart contract engine  
- Persistent storage  

---

# 5. Non-Goals

The following are *not* goals of this project:

- Production-ready security  
- Economic incentives  
- Full cryptographic wallet infrastructure  
- Scalability or optimization  
- Proof-of-Stake or alternate consensus  

This is a **learning-focused blockchain core**.

---

# 6. Future Enhancements

See the project **ROADMAP** in `README.md` for specific development milestones.