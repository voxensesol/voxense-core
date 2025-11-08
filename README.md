<p align="center">
  <img src="https://i.imgur.com/NWptykH.png" width="100%" alt="VOXENSE Banner"/>
</p>

<h1 align="center">🛰️ VOXENSE CORE</h1>

<p align="center">
  <b>The Proof-of-Sensing Protocol</b><br/>
  <i>Core smart contracts powering the Real-World Sensor Network on Solana.</i>
</p>

---

<p align="center">
  <img src="https://img.shields.io/github/license/voxensesol/voxense-core?style=flat-square"/>
  <img src="https://img.shields.io/badge/Network-Solana-9945FF?style=flat-square"/>
  <img src="https://img.shields.io/badge/DePIN-Enabled-C0FF00?style=flat-square"/>
  <img src="https://img.shields.io/badge/Status-Beta-green?style=flat-square"/>
</p>

---

## 🧠 Overview
**VOXENSE Core** is the foundational layer of the **VOXENSE Network** — a Spatial DePIN protocol that verifies real-world sensing through on-chain cryptographic proof.

Each connected node (mobile or IoT) produces sensor hashes (GPS, Audio, Ambient, Motion) that are validated by the **Proof-of-Sensing (PoS)** engine, rewarded in `$VOX`, and stored as verified reality events.

> *When the environment agrees, the network believes.*

---

## ⚙️ Core Modules

| Module | Purpose |
|--------|----------|
| **PoS Engine** | Validates multi-sensor data between nodes |
| **Reward Distributor** | Allocates $VOX rewards to verified contributors |
| **Node Registry** | Tracks unique node fingerprints and activity |
| **Spatial Ledger** | Immutable record of verified sensing proofs |
| **Reputation System** | Scores node trustworthiness based on accuracy |

---

## 🧱 Architecture
[Sensors: GPS / Audio / Motion / Ambient]
↓ (data hash)
[Voxense SDK]
↓
[PoS Engine]
↓
[Solana Smart Contracts]
↓
[Reality Layer & Marketplace]


---

## 💰 Token Utility — `$VOX`
| Action | Reward | Purpose |
|--------|---------|---------|
| Submit Sensor Hash | +0.05 VOX | Proof contribution |
| Verified Consensus | +0.1 VOX | Environmental agreement |
| Node Uptime | +0.2 VOX | Reliability reward |
| Data Access Fees | Variable | Marketplace revenue |

---

## 🧩 Example Contract Stub (Rust / Anchor)
```rust
use anchor_lang::prelude::*;

declare_id!("VoXense1111111111111111111111111111111111");

#[program]
pub mod voxense_core {
    use super::*;

    pub fn register_node(ctx: Context<RegisterNode>, node_id: Pubkey) -> Result<()> {
        let node = &mut ctx.accounts.node;
        node.owner = *ctx.accounts.authority.key;
        node.node_id = node_id;
        Ok(())
    }

    pub fn submit_proof(ctx: Context<SubmitProof>, data_hash: [u8; 32]) -> Result<()> {
        let proof = &mut ctx.accounts.proof;
        proof.data_hash = data_hash;
        proof.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[account]
pub struct Node {
    pub owner: Pubkey,
    pub node_id: Pubkey,
}

#[account]
pub struct Proof {
    pub data_hash: [u8; 32],
    pub timestamp: i64,
}
