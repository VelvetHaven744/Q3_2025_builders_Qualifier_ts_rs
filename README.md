# 🚀 Q3 2025 Builders Qualifier – Rust & TypeScript on Solana

This repository contains my solutions for the **Q3 2025 Builders Qualifier prerequisites**, implemented in **Rust (Anchor)** and **TypeScript** to showcase end‑to‑end interaction with the **Solana blockchain**.

---

## 🏗 Project Structure
Q3_2025_builders_Qualifier_ts_rs/
├── airdrop_rs/ # Rust implementation (Anchor program + helpers)
│ ├── src/
│ │ └── lib.rs # Main Rust library & unit‑tests
│ ├── Cargo.toml # Rust dependencies
│ └── target/ # Build artifacts (git‑ignored)
│
├── airdrop_ts/ # TypeScript implementation (client scripts)
│ ├── programs/
│ │ └── Turbin3_prereq.ts # Turbin3 prerequisite logic
│ ├── *.ts # Various helper scripts
│ ├── package.json # Node / Yarn deps
│ └── node_modules/ # Installed deps (git‑ignored)
│
├── idl/ # IDL JSON copied from target/idl (for TS client)
├── .gitignore
└── README.md

yaml
Copy
Edit

---

## ✨ Features

| Module | Highlights |
|--------|------------|
| **Rust (`airdrop_rs/`)** | 🔑 Keypair generation, 🔁 Base58 ↔ wallet conversion, 🪂 devnet airdrops, 💸 SOL transfers & full‑balance sweep, 📤 submission to the Turbin3 prerequisite program |
| **TypeScript (`airdrop_ts/`)** | 🔑 Keypair generation, 🪂 airdrops, 💸 transfers (single & full balance), 📝 enrollment script, 🔁 key‑format conversion, 🤝 high‑level Turbin3 helpers |

---

## 📋 Prerequisites

| Stack | Requirement |
|-------|-------------|
| **Rust** | `rustup` (latest stable), `cargo`, Anchor CLI |
| **TypeScript** | Node ≥ v16, Yarn or npm |

---

## 🛠 Installation & Setup

### Rust
```bash
cd airdrop_rs
cargo build          # builds the Anchor program
TypeScript
bash
Copy
Edit
cd airdrop_ts
yarn install         # or: npm install
🎯 Usage
Rust (inside airdrop_rs/)
Purpose	Command
Generate keypair	cargo test keygen
Base58 → wallet	cargo test base58_to_wallet
Wallet → Base58	cargo test wallet_to_base58
Claim airdrop	cargo test claim_airdrop
Transfer 0.1 SOL	cargo test transfer_sol
Transfer all	cargo test transfer_all
Submit to Turbin3	cargo test submit_rs

TypeScript (inside airdrop_ts/)
Purpose	Command
Generate keypair	yarn keygen
Claim airdrop	yarn airdrop
Transfer SOL	yarn transfer
Transfer all	yarn transfer-all
Enroll program	yarn enroll
Convert keys	yarn convert

🔧 Configuration
Item	File	Notes
Dev wallet	dev-wallet.json	Used for airdrop & testing
Turbin3 wallet	Turbin3-wallet.json	Used when interacting with Turbin3 contracts
Rust RPC	https://turbine-solanad-4cde.devnet.rpcpool.com/	
TypeScript RPC	https://api.devnet.solana.com	

📝 Key Functions
Rust (lib.rs)
keygen() – create a new wallet

base58_to_wallet() / wallet_to_base58() – format conversions

claim_airdrop() – request 2 SOL on devnet

transfer_sol() – send 0.1 SOL to Turbin3 wallet

transfer_all() – sweep entire balance

submit_rs() – submit proof to Turbin3 program

TypeScript
keygen.ts, airdrop.ts, transfer.ts, transfer-all.ts, enroll.ts, key-converter.ts

programs/Turbin3_prereq.ts – Anchor client wrapper

🔐 Security Notes
Wallet JSON files contain private keys → they’re .gitignored by default.

Always test on devnet first; never expose these keys on mainnet.

The repo’s .gitignore blocks *-wallet.json, target/, and node_modules/.

🌐 Network
Item	URL
Devnet Explorer	https://explorer.solana.com/?cluster=devnet
Rust RPC	https://turbine-solanad-4cde.devnet.rpcpool.com/
TS RPC	https://api.devnet.solana.com

📦 Dependencies
Rust
solana-sdk, solana-client, solana-program, bs58, borsh, solana-idlgen

TypeScript
@solana/web3.js, @coral-xyz/anchor, bs58, prompt-sync, typescript

🤝 Contributing
Fork the repo and create a feature branch

Implement your change (and tests) in both stacks if relevant

Run cargo test and yarn test (if configured)

Open a PR describing your changes

📄 License
This repository is published solely for the Q3 2025 Builders Qualifier.
Use at your own risk; always keep private keys secure.

🔗 Useful Links
Solana Docs

Anchor Framework

Solana Devnet Status

Turbin3 program guidelines (internal link)
