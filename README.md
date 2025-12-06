# ArcNova Chain

A lightweight blockchain core written in Rust.  
Designed to demonstrate blockchain fundamentals in a clear, hands-on way.

> **Note:** This is just a very lightweight demo of the full project.  
> We'll keep improving it over time, and contributions are always welcome — they might even lead to some exciting opportunities!

**Key concepts explored:**

- Blocks and Proof-of-Work (PoW) mining
- Merkle trees for transaction integrity
- Wallets with public/private keys (Ed25519)
- Signed transactions
- Full chain validation

---

## ✨ Features

### 🔹 Block Structure
- Index, timestamp, previous hash  
- Proof-of-Work difficulty & nonce  
- Merkle root of transactions  

### 🔹 Proof-of-Work
- Adjustable difficulty  
- Mining loop that finds a hash target beginning with `0000...`  
- Block hash includes:
  - index
  - timestamp
  - merkle root
  - previous hash
  - nonce
  - difficulty

### 🔹 Transactions
- Multiple transactions per block  
- Includes sender, recipient, amount  
- Signed using Ed25519 private keys  
- Signature verification included  

### 🔹 Wallets
- Generate public/private keypairs  
- SHA-256–based address creation  
- Sign and verify transaction messages  

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/stunkki/arcnova-chain.git
cd arcnova-chain

# Build and run
cargo run --release

ArcNova Chain includes build metadata in the binary, so you can see for example the **Git commit SHA** and **build timestamp**.