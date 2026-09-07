# CKB Digital Asset Ownership dApp

A CKB Testnet decentralized application for learning how digital asset ownership can be represented, transferred, and validated using **Nervos CKB Cells, transactions, Rust scripts, and CCC wallet integration**.

This project is part of **Week 7 of the CKB Learning Journey**.

---

## Overview

The goal of this project is to move from writing isolated CKB/Rust contracts into building a complete decentralized application that interacts with the CKB blockchain.

The application explores a simple digital asset ownership model:

```text
Asset
 ├── Asset ID
 ├── Owner
 ├── Metadata
 └── Version
```

Ownership is represented through a CKB Cell.

The intended lifecycle is:

```text
Create Asset
     │
     ▼
Asset Cell
     │
     ▼
Current Owner
     │
     │ Transfer
     ▼
New Asset Cell
     │
     ▼
New Owner
```

The Rust contract is responsible for validating that the asset state is not modified incorrectly during a transaction.

---

## Learning Objectives

This project demonstrates:

* CKB Cells
* CKB transaction construction
* CKB ownership through lock scripts
* CKB type scripts
* Cell dependencies
* Transaction inputs and outputs
* CCC wallet integration
* CKB Testnet transactions
* Rust-based transaction validation
* Digital asset state representation
* Ownership transfer
* Fraudulent transaction detection
* Client-side blockchain interaction
* Next.js App Router
* TypeScript

---

## Technology Stack

### Frontend

* Next.js
* React
* TypeScript
* Tailwind CSS

### Blockchain

* Nervos CKB
* CKB Testnet
* CKB Cells
* CKB Transactions
* CKB Scripts

### Wallet / SDK

* `@ckb-ccc/connector-react`
* CCC

### Smart Contract

* Rust
* CKB RISC-V execution environment

---

## Project Structure

```text
digital-asset-app/
│
├── app/
│   ├── globals.css
│   ├── layout.tsx
│   ├── page.tsx
│   └── providers.tsx
│
├── components/
│   ├── AssetCard.tsx
│   ├── ConnectWallet.tsx
│   ├── CreateAsset.tsx
│   ├── Dashboard.tsx
│   └── TransferAsset.tsx
│
├── lib/
│   ├── asset.ts
│   ├── ckb.ts
│   └── types.ts
│
├── public/
│   └── icon.svg
│
├── package.json
├── next.config.ts
├── tsconfig.json
└── README.md
```

The Rust contract is maintained separately:

```text
contracts/
└── digital-asset/
    ├── Cargo.toml
    ├── Makefile
    ├── README.md
    └── src/
        ├── lib.rs
        ├── main.rs
        ├── asset.rs
        ├── errors.rs
        └── validator.rs
```

---

# Architecture

The application consists of three main layers.

```text
┌─────────────────────────────┐
│          Next.js            │
│                             │
│  Create / View / Transfer   │
└──────────────┬──────────────┘
               │
               │ CCC
               ▼
┌─────────────────────────────┐
│        CKB Wallet           │
│                             │
│     Sign Transaction        │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│        CKB Testnet          │
│                             │
│       Asset Cells           │
│       Transactions          │
│       Rust Scripts          │
└─────────────────────────────┘
```

---

# Asset Model

Each digital asset is represented by data stored in a CKB Cell.

The conceptual structure is:

```text
Asset Cell
│
├── Capacity
│
├── Lock Script
│      └── Current owner
│
├── Type Script
│      └── Digital Asset validator
│
└── Data
       ├── Version
       ├── Asset ID
       ├── Owner information
       └── Metadata hash
```

The current learning encoding is:

```text
version       1 byte
asset_id     32 bytes
owner        20 bytes
metadata     32 bytes
```

Total:

```text
85 bytes
```

---

# Asset Data Encoding

The frontend contains the following functions:

```text
encodeAssetData()
decodeAssetData()
```

The encoded structure is:

```text
┌─────────┬────────────────┬──────────────┬────────────────┐
│ Version │    Asset ID    │    Owner     │ Metadata Hash  │
│ 1 byte  │    32 bytes    │   20 bytes   │    32 bytes    │
└─────────┴────────────────┴──────────────┴────────────────┘
```

This allows the application to convert between a JavaScript representation and the raw hexadecimal data stored in the CKB Cell.

---

# Wallet Connection

The application uses CCC to connect to a user's CKB wallet.

The provider is configured at the root of the application:

```tsx
<ccc.Provider
  name="CKB Digital Asset Lab"
  icon="/icon.svg"
>
  {children}
</ccc.Provider>
```

Wallet state is accessed through:

```ts
ccc.useCcc()
```

The application can obtain:

* connected wallet
* wallet name
* signer
* recommended address
* balance

---

# Network

The project is designed for **CKB Testnet**.

This is intentional.

No mainnet assets should be used while developing or testing the application.

Before performing transactions, make sure the connected wallet is operating on the CKB Testnet network.

---

# Transaction Flow

A simplified CKB transaction looks like:

```text
Transaction
│
├── Inputs
│      └── Existing Cells
│
├── Outputs
│      └── New Cells
│
├── Outputs Data
│      └── Asset state
│
├── Cell Deps
│      └── Script dependencies
│
└── Witnesses
       └── Signatures
```

The frontend uses CCC to construct and sign transactions.

The basic transaction lifecycle is:

```text
Transaction.from(...)
        │
        ▼
completeInputsByCapacity()
        │
        ▼
completeFeeBy()
        │
        ▼
signer.sendTransaction()
        │
        ▼
CKB Testnet
```

---

# Creating an Asset

The application generates:

```text
Asset ID
Metadata Hash
```

and encodes them into Cell data.

A transaction is then constructed with an output Cell.

Conceptually:

```text
User Wallet
     │
     │ Sign
     ▼
Transaction
     │
     ├── Input: user's CKB
     │
     └── Output:
           ├── owner lock
           └── asset data
```

The transaction is then broadcast to CKB Testnet.

---

# Transferring an Asset

The intended final transfer flow is:

```text
Alice owns Asset #001

Asset Cell
Owner = Alice
     │
     │ consume
     ▼
Rust validator
     │
     │ validate
     ▼
New Asset Cell
Owner = Bob
```

The transfer must preserve the asset's identity and immutable metadata.

Only the ownership state should change.

---

# Ownership Validation

The Rust contract is intended to enforce rules such as:

```text
Asset ID must remain unchanged
Metadata hash must remain unchanged
Asset version must remain valid
Exactly one asset state must be preserved
Ownership transition must be valid
```

For example:

```text
Input:

Asset ID = A
Metadata = M
Owner = Alice

Output:

Asset ID = A
Metadata = M
Owner = Bob

Result:

VALID
```

But:

```text
Input:

Asset ID = A
Metadata = M
Owner = Alice

Output:

Asset ID = B
Metadata = M
Owner = Bob

Result:

INVALID
```

Likewise:

```text
Input:

Asset ID = A
Metadata = M
Owner = Alice

Output:

Asset ID = A
Metadata = X
Owner = Bob

Result:

INVALID
```

---

# Fraudulent Transaction Scenarios

An important part of Week 7 is understanding that the frontend cannot be trusted.

A malicious user could attempt to construct a transaction manually.

Examples include:

### 1. Changing the Asset ID

```text
Input Asset ID  = A
Output Asset ID = B
```

Should fail.

### 2. Changing Metadata

```text
Input Metadata  = Original
Output Metadata = Fake
```

Should fail.

### 3. Creating a duplicate asset

```text
Asset #001
Asset #001
```

The contract must prevent invalid duplication according to the chosen asset model.

### 4. Unauthorized ownership transition

```text
Alice → Bob
```

must only be valid when the transaction satisfies the ownership rules.

### 5. Malformed asset data

An attacker could submit:

```text
0x1234
```

instead of the expected 85-byte structure.

The validator should reject malformed data.

---

# Important Current Implementation Status

The current frontend contains a **real CCC wallet connection and real CKB Testnet transaction flow**.

However, the custom Rust digital-asset type script still needs to be deployed before the application can claim that the Rust contract itself is enforcing the ownership rules on-chain.

Therefore:

```text
Wallet connection       ✅
CCC integration         ✅
Testnet transaction     ✅
Asset data encoding     ✅
UI                      ✅
Rust contract design    ✅
Rust contract tests     🔄
Contract deployment     🔄
Type script integration 🔄
Real asset-cell transfer 🔄
On-chain fraud testing  🔄
```

This distinction is important for maintaining a technically accurate project.

---

# Installation

Clone the repository and enter the dApp directory:

```bash
cd dapps/digital-asset-app
```

Install dependencies:

```bash
npm install
```

Run the development server:

```bash
npm run dev
```

Open:

```text
http://localhost:3000
```

---

# Build

Create a production build:

```bash
npm run build
```

Run the production server:

```bash
npm start
```

---

# Testing Checklist

Before considering Week 7 complete, the following should be tested.

## Wallet

* [ ] Connect wallet
* [ ] Disconnect wallet
* [ ] Display wallet address
* [ ] Display CKB balance
* [ ] Confirm Testnet network

## Asset Creation

* [ ] Generate asset ID
* [ ] Generate metadata hash
* [ ] Encode asset data
* [ ] Construct transaction
* [ ] Sign transaction
* [ ] Broadcast transaction
* [ ] Verify transaction on explorer

## Asset Transfer

* [ ] Locate asset Cell
* [ ] Consume existing asset Cell
* [ ] Create new asset Cell
* [ ] Change ownership
* [ ] Preserve asset ID
* [ ] Preserve metadata
* [ ] Attach correct cell dependency
* [ ] Sign transaction
* [ ] Broadcast transaction

## Security

* [ ] Reject modified asset ID
* [ ] Reject modified metadata
* [ ] Reject malformed asset data
* [ ] Reject invalid ownership transition
* [ ] Test duplicate/invalid asset states
* [ ] Test fraudulent transaction attempts

---

# Learning Outcomes

By completing this project, I should be able to explain:

1. How CKB represents state using Cells.
2. How ownership can be represented using lock scripts.
3. How type scripts validate transaction state transitions.
4. Why the frontend cannot enforce blockchain security.
5. How a CKB transaction consumes inputs and creates outputs.
6. Why Cell dependencies are required for script execution.
7. How Rust scripts run inside the CKB execution environment.
8. How CCC connects a web application to CKB wallets.
9. How digital asset ownership can be modeled using CKB's Cell model.
10. How fraudulent transactions can be rejected at the protocol level.

---

# Future Improvements

Possible improvements include:

* Asset marketplace
* Asset listing and discovery
* Asset metadata stored using content-addressed storage
* NFT-style visual assets
* Asset history
* Transaction history
* CKB Indexer integration
* Better asset querying
* Multiple asset types
* Asset burning
* Asset freezing
* Admin-controlled collections
* Spore Protocol integration
* Production deployment

---

# Educational Disclaimer

This project is an educational implementation created while learning Nervos CKB.

It should not be treated as production-ready digital asset infrastructure until the contract has been thoroughly tested, audited, and deployed appropriately.

---

# Week 7

**Project:** CKB Digital Asset Ownership dApp

**Network:** CKB Testnet

**Focus:**

```text
CKB Cells
+
Transactions
+
Rust Scripts
+
Wallets
+
Digital Asset Ownership
+
Fraud Prevention
```
