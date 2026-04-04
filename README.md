# demo-project
# 🗳️ Permissionless Voting DApp on Stellar

A fully decentralized and permissionless voting application built on the Stellar blockchain using Soroban smart contracts.

---

## 🚀 Overview

This project demonstrates how to build a **trustless voting system** where:

* Anyone can create a poll
* Anyone can vote
* No admin or central authority exists
* All data is transparent and immutable

Built using **Stellar Soroban smart contracts**, this DApp ensures fairness and openness by design.

---

## 🔥 Key Features

* 🆓 **Permissionless Poll Creation**
  Any user can create a new voting poll without approval.

* 🗳️ **Open Voting System**
  Anyone can vote in any poll.

* 🔁 **One Vote Per User Per Poll**
  Prevents duplicate voting while remaining decentralized.

* 🔍 **Transparent Results**
  Votes are stored on-chain and can be verified by anyone.

* ⚡ **Fast & Low Cost**
  Powered by Stellar for quick and cheap transactions.

---

## 🧠 Smart Contract Logic

### Core Functions

* `create_poll(title)`
  Creates a new poll with a unique ID.

* `vote(poll_id, option)`
  Cast a vote for a given option.

* `get_results(poll_id)`
  Returns vote counts for each option.

* `has_voted(poll_id, user)`
  Checks if a user has already voted.

---

## 🔐 Permissionless Design

This DApp strictly follows a **permissionless architecture**:

* ❌ No admin controls
* ❌ No candidate approval system
* ❌ No whitelist/blacklist

✅ Anyone can:

* Create polls
* Participate in voting
* View results

---

## 🛠️ Tech Stack

* **Blockchain:** Stellar (Soroban)
* **Smart Contracts:** Rust (Soroban SDK)
* **Frontend:** React.js
* **Wallet:** Freighter Wallet

---

## 📦 Project Structure

```
├── contracts/
│   └── voting_contract/
│       └── src/
│           └── lib.rs
├── frontend/
│   ├── src/
│   └── components/
├── README.md
```

---

## ⚙️ Installation & Setup

### 1. Clone Repository

```bash
git clone https://github.com/your-username/stellar-voting-dapp.git
cd stellar-voting-dapp
```

---

### 2. Install Dependencies

#### Smart Contract (Rust)

```bash
cd contracts/voting_contract
cargo build
```

---

### 3. Deploy Contract (Testnet)

Make sure you have installed:

* Soroban CLI
* Stellar CLI

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/voting_contract.wasm \
--source your-account
```

---

### 4. Run Frontend

```bash
cd frontend
npm install
npm start
```

---

## 🌐 Usage

1. Connect your wallet
2. Create a new poll
3. Share poll ID
4. Users vote
5. View results instantly

---

## 🎨 UI Branding

* **App Name:** OpenVote

* **Hero Text:**
  *"Vote Freely. No Permission. No Control."*

* **Navbar:**

  * Create Poll
  * Vote
  * Results

---

## 💡 Future Improvements

* 📊 Real-time analytics dashboard
* 🧾 NFT-based voting proof
* 🔐 Optional private voting (ZK proofs)
* ⏳ Time-bound polls
* 🌍 Multi-language support


source code : https://stellar.expert/explorer/testnet/contract/CCUIB74RE5D7ZN5GL3OVQEGW7JYFYOEJAT5N2N2WD5TCVZIQMU5QRLCF
ID : CCUIB74RE5D7ZN5GL3OVQEGW7JYFYOEJAT5N2N2WD5TCVZIQMU5QRLCF

<img width="1901" height="1029" alt="Screenshot 2026-04-04 095203" src="https://github.com/user-attachments/assets/78cb9c5e-98af-4237-b04d-d8625f69b1c5" />
