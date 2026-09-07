# Week 7 — Building a CKB Digital Asset Ownership dApp

**CKB Learning Journey**

**Week:** 7
**Focus:** Digital Asset Ownership
**Project:** CKB Digital Asset Lab
**Network:** CKB Testnet

---

## 1. Week Overview

Week 7 marked a transition from studying individual CKB concepts to combining them into a complete decentralized application.

The objective was not simply to create another frontend interface.

Instead, the goal was to understand how a real application can interact with CKB:

```text
User
 │
 ▼
Web Application
 │
 ▼
CKB Wallet
 │
 ▼
Signed Transaction
 │
 ▼
CKB Testnet
 │
 ▼
Cells + Scripts
```

The project chosen for this week was a **Digital Asset Ownership dApp**.

The application allows a user to connect a CKB wallet, inspect their wallet information, create an asset transaction, and prepare an ownership-transfer workflow.

The deeper objective was to understand how ownership can be represented as state within CKB Cells and how Rust scripts can enforce valid state transitions.

---

# 2. Why Digital Asset Ownership?

Digital assets provide a practical way to understand the CKB Cell model.

A digital asset can be represented as:

```text
Asset
├── Identity
├── Owner
├── Metadata
└── Version
```

The important question was:

> How can CKB guarantee that someone cannot simply modify the asset's identity or metadata when transferring ownership?

This led directly into the relationship between:

```text
Cells
   +
Transactions
   +
Lock Scripts
   +
Type Scripts
   +
Rust
```

Rather than treating the blockchain as a database, I approached the problem as a **state-transition system**.

---

# 3. Project Architecture

The final architecture was designed around three layers.

```text
┌───────────────────────────────┐
│           Next.js             │
│                               │
│ Dashboard                     │
│ Create Asset                  │
│ Asset Display                 │
│ Transfer Asset                │
└───────────────┬───────────────┘
                │
                ▼
┌───────────────────────────────┐
│             CCC               │
│                               │
│ Wallet connection             │
│ Signer                        │
│ Transaction construction      │
└───────────────┬───────────────┘
                │
                ▼
┌───────────────────────────────┐
│          CKB Testnet          │
│                               │
│ Asset Cells                   │
│ Transactions                  │
│ Cell Dependencies             │
│ Rust Scripts                  │
└───────────────────────────────┘
```

This architecture helped separate application logic from blockchain validation.

---

# 4. Technology Used

The frontend was built using:

* Next.js
* React
* TypeScript
* Tailwind CSS

The blockchain integration uses:

* Nervos CKB
* CKB Testnet
* CCC
* CKB transactions
* CKB Cells

The custom validator is being developed using:

* Rust
* CKB Script environment
* RISC-V

---

# 5. Creating the Project

The dApp was created under:

```text
dapps/digital-asset-app
```

The project structure became:

```text
dapps/
└── digital-asset-app/
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
    └── README.md
```

This was intentionally kept modular rather than placing the entire application in a single page.

---

# 6. CCC Wallet Integration

One of the biggest practical improvements this week was moving beyond simulated blockchain interaction.

The application uses CCC to connect to a CKB wallet.

The provider is placed at the root:

```tsx
<ccc.Provider
  name="CKB Digital Asset Lab"
  icon="/icon.svg"
>
```

The application can then use:

```ts
ccc.useCcc()
```

and:

```ts
ccc.useSigner()
```

to access wallet and signing functionality.

This means the application can interact with a real wallet instead of pretending to perform blockchain transactions.

---

# 7. Wallet Dashboard

The dashboard displays:

```text
Connected wallet
Testnet balance
```

The signer is used to obtain:

```text
Recommended address
Balance
```

The important lesson here was that blockchain applications should not assume that a wallet address is available.

The UI must handle:

```text
Wallet disconnected
        │
        ▼
Connect wallet
        │
        ▼
Signer available
        │
        ▼
Blockchain operations enabled
```

---

# 8. Understanding the Asset Cell

The central idea of the project is the Asset Cell.

Conceptually:

```text
┌────────────────────────────┐
│         Asset Cell         │
├────────────────────────────┤
│ Capacity                   │
├────────────────────────────┤
│ Lock Script                │
│                            │
│ Current Owner              │
├────────────────────────────┤
│ Type Script                │
│                            │
│ Digital Asset Validator    │
├────────────────────────────┤
│ Data                       │
│                            │
│ Version                    │
│ Asset ID                   │
│ Owner                      │
│ Metadata Hash              │
└────────────────────────────┘
```

This was an important conceptual shift.

Instead of thinking:

```text
Database:

asset.owner = Alice
```

the CKB approach is closer to:

```text
Current Cell
     │
     │ consumed
     ▼
New Cell
```

The blockchain state changes by consuming and creating Cells.

---

# 9. Asset Data Design

The project uses an 85-byte asset data structure.

```text
Version       1 byte
Asset ID     32 bytes
Owner        20 bytes
Metadata     32 bytes
```

Therefore:

```text
1 + 32 + 20 + 32 = 85 bytes
```

The frontend provides:

```ts
encodeAssetData()
```

and:

```ts
decodeAssetData()
```

to convert between structured asset information and raw blockchain data.

The format is:

```text
┌─────────┬────────────────┬──────────────┬────────────────┐
│ Version │    Asset ID    │    Owner     │ Metadata Hash  │
│ 1 byte  │    32 bytes    │   20 bytes   │    32 bytes    │
└─────────┴────────────────┴──────────────┴────────────────┘
```

This gave the project a deterministic representation of asset state.

---

# 10. Creating an Asset

The creation flow is:

```text
Connect Wallet
      │
      ▼
Get Owner Address
      │
      ▼
Generate Asset ID
      │
      ▼
Generate Metadata Hash
      │
      ▼
Encode Asset Data
      │
      ▼
Construct Transaction
      │
      ▼
Complete Inputs
      │
      ▼
Complete Fee
      │
      ▼
Wallet Signature
      │
      ▼
Broadcast
```

This was the first point where the application became a real CKB transaction client.

---

# 11. Transaction Construction

The transaction model helped reinforce the structure of CKB transactions:

```text
Transaction
│
├── Inputs
│
├── Outputs
│
├── Outputs Data
│
├── Cell Deps
│
└── Witnesses
```

The frontend uses CCC to construct a transaction.

The general sequence is:

```ts
ccc.Transaction.from(...)
```

followed by:

```ts
tx.completeInputsByCapacity(...)
```

and:

```ts
tx.completeFeeBy(...)
```

before finally:

```ts
signer.sendTransaction(tx)
```

The important lesson was that creating an output is not enough.

A transaction must also contain enough inputs to fund its capacity and enough fee to be accepted.

---

# 12. Ownership Transfer

The intended transfer model is:

```text
Alice
 │
 │ owns
 ▼
Asset Cell
 │
 │ consume
 ▼
Transaction
 │
 │ validate
 ▼
New Asset Cell
 │
 │ owner
 ▼
Bob
```

The key invariant is:

```text
Asset ID:       unchanged
Metadata:       unchanged
Ownership:      changed
```

For example:

```text
Before:

Asset ID = A
Owner    = Alice
Metadata = M

After:

Asset ID = A
Owner    = Bob
Metadata = M
```

This is a valid state transition.

---

# 13. Fraudulent Transfer

One of the most important parts of Week 7 was considering what happens when a malicious user does not use the frontend.

A blockchain application cannot rely on:

```text
React validation
```

or:

```text
TypeScript validation
```

for security.

An attacker can construct their own transaction.

Therefore the important security logic must exist on-chain.

---

## Attack 1 — Change Asset ID

Valid:

```text
Input:
Asset ID = A

Output:
Asset ID = A
```

Invalid:

```text
Input:
Asset ID = A

Output:
Asset ID = B
```

The validator should reject the transaction.

---

## Attack 2 — Modify Metadata

Valid:

```text
Input metadata  = M
Output metadata = M
```

Invalid:

```text
Input metadata  = M
Output metadata = X
```

The asset's metadata should remain immutable if the asset model defines it as immutable.

---

## Attack 3 — Malformed Data

An attacker could submit:

```text
0x1234
```

instead of the expected asset structure.

The validator should check the data length before attempting to parse it.

---

## Attack 4 — Invalid Ownership Transition

The attacker could attempt to create an output representing an asset they should not be able to transfer.

The contract must validate the relationship between the input state, transaction structure, and resulting output state.

---

# 14. Why the Rust Script Matters

This project helped clarify the difference between frontend logic and blockchain consensus.

Frontend:

```text
"Transfer button is enabled."
```

does not mean:

```text
"The blockchain accepts this transfer."
```

The actual security boundary is:

```text
Transaction
     │
     ▼
CKB Script
     │
     ├── valid → transaction accepted
     │
     └── invalid → transaction rejected
```

This is one of the most important lessons from this week.

---

# 15. Cell Dependencies

Another important concept was the role of `cellDeps`.

A transaction that needs to execute a particular script must make the script code available through its dependencies.

Conceptually:

```text
Transaction
│
├── Input
│
├── Output
│
├── Cell Dep
│     │
│     ▼
│   Script Code
│
└── Witness
```

Therefore, after deploying the custom Rust contract, the dApp will need to reference its deployed script through a `cellDep`.

---

# 16. Current Implementation Status

The project intentionally distinguishes between what is already functional and what still requires deployment.

### Completed

```text
Next.js application             ✅
TypeScript                      ✅
CCC provider                    ✅
Wallet connection               ✅
Wallet disconnection            ✅
Wallet address                  ✅
Wallet balance                  ✅
Asset data encoding             ✅
Transaction construction        ✅
CKB Testnet transaction flow    ✅
Asset UI                        ✅
Create asset workflow           ✅
```

### Remaining

```text
Rust contract finalization      🔄
Rust validator tests            🔄
Contract deployment             🔄
Script code hash                🔄
Cell dependency configuration   🔄
Asset Cell discovery            🔄
True asset-cell transfer        🔄
Fraudulent transaction tests    🔄
```

This distinction is important because a transaction that contains asset data is not automatically protected by the custom Rust contract.

---

# 17. What I Learned

## Lesson 1 — CKB is fundamentally Cell-based

The biggest conceptual takeaway was that CKB state is represented by Cells rather than conventional mutable database rows.

Instead of:

```text
UPDATE asset
SET owner = Bob
```

the conceptual model is:

```text
Consume Alice's Cell
          │
          ▼
Create Bob's Cell
```

---

## Lesson 2 — Transactions describe state transitions

A transaction is not simply a payment.

It can describe:

```text
Old State
    ↓
Validation
    ↓
New State
```

This makes CKB particularly interesting for custom asset models.

---

## Lesson 3 — Type scripts enforce rules

The frontend can construct transactions, but the type script determines whether a particular state transition is valid.

This gives the application a much stronger security boundary.

---

## Lesson 4 — Wallets are signing interfaces

The dApp does not need access to the user's private key.

Instead:

```text
dApp
 │
 │ transaction
 ▼
Wallet
 │
 │ signature
 ▼
dApp
 │
 ▼
CKB
```

This is the correct model for a browser-based blockchain application.

---

## Lesson 5 — Blockchain security cannot depend on the UI

A user can bypass:

```text
React
TypeScript
Buttons
Forms
Client-side validation
```

Therefore the important rules must ultimately be enforced by the blockchain scripts.

---

# 18. Challenges

One of the main challenges was understanding where application logic ends and blockchain validation begins.

It is easy to build:

```text
Create Asset
Transfer Asset
```

buttons.

It is much harder to make the resulting transaction represent a valid CKB state transition.

Another challenge is that deploying a Rust contract introduces additional concepts:

```text
RISC-V binary
Code hash
Script hash
Cell dependency
Type script
Out point
Transaction
```

Understanding these pieces is necessary before the custom validator can be safely connected to the frontend.

---

# 19. Security Model

The intended security model is:

```text
                    User
                     │
                     ▼
               CKB Wallet
                     │
                     ▼
              Signed Transaction
                     │
                     ▼
              CKB Validation
                     │
              ┌──────┴──────┐
              │             │
            Valid         Invalid
              │             │
              ▼             ▼
           Accepted       Rejected
```

The browser is therefore treated as untrusted.

The contract is the final authority for asset state transitions.

---

# 20. Planned Final Transaction

Once the Rust contract is deployed, the final transfer transaction should resemble:

```text
┌────────────────────────────────────────┐
│              Transaction               │
├────────────────────────────────────────┤
│                                        │
│ Inputs                                 │
│ └── Existing Asset Cell                │
│                                        │
│ Cell Deps                              │
│ └── Digital Asset Script               │
│                                        │
│ Outputs                                │
│ └── New Asset Cell                     │
│      ├── New owner lock                │
│      ├── Same type script              │
│      └── Same asset state              │
│                                        │
│ Witnesses                              │
│ └── Owner signature                    │
│                                        │
└────────────────────────────────────────┘
```

The Rust validator will compare the relevant input and output state.

---

# 21. Fraud Testing Plan

Before considering the contract complete, the following malicious cases should be tested.

### Test 1

```text
Change Asset ID
→ reject
```

### Test 2

```text
Change immutable metadata
→ reject
```

### Test 3

```text
Malformed asset data
→ reject
```

### Test 4

```text
Missing asset input
→ reject
```

### Test 5

```text
Invalid output asset state
→ reject
```

### Test 6

```text
Unauthorized transfer
→ reject
```

### Test 7

```text
Valid owner transfer
→ accept
```

The purpose of these tests is to demonstrate that the security model works even when the attacker completely bypasses the web application.

---

# 22. Week 7 Achievement

By the end of this week, the project has moved from:

```text
Learning CKB concepts
```

to:

```text
Building a CKB application
```

The application now has a real blockchain interaction layer:

```text
Next.js
   ↓
CCC
   ↓
CKB Wallet
   ↓
CKB Testnet
```

The next stage is to complete the custom contract integration:

```text
Next.js
   ↓
CCC
   ↓
Transaction
   ↓
Digital Asset Type Script
   ↓
Rust Validator
   ↓
CKB Consensus
```

---

# 23. Reflection

Week 7 was important because it connected the theoretical concepts from the previous weeks to an actual application.

Previously, the focus was primarily on understanding:

```text
CKB
Cells
Transactions
Rust
Scripts
Validation
```

This week required combining them.

The most important realization was that building a blockchain application is not simply about connecting a wallet and sending a transaction.

The difficult and interesting part is defining **what constitutes a valid state transition**.

For the digital asset project, that means ensuring:

```text
Asset identity remains stable
        +
Metadata remains valid
        +
Ownership changes correctly
        +
Invalid transitions are rejected
```

That is where the Rust contract becomes essential.

---

# 24. Next Week

The immediate next step is to finish the custom digital asset validator and deploy it to CKB Testnet.

The workflow will be:

```text
Write Rust Validator
        ↓
Write Fraud Tests
        ↓
Build RISC-V Binary
        ↓
Deploy Contract
        ↓
Get Code Hash
        ↓
Get Deployment OutPoint
        ↓
Configure Cell Dep
        ↓
Update dApp
        ↓
Create Typed Asset Cell
        ↓
Query Asset Cell
        ↓
Consume Asset Cell
        ↓
Transfer Ownership
        ↓
Test Fraudulent Transactions
```

After that, the project can evolve toward a more complete digital asset protocol or be compared with existing CKB standards such as Spore.

---

# 25. Final Week 7 Summary

**Project:** CKB Digital Asset Ownership dApp

**Network:** CKB Testnet

**Primary Technologies:**

```text
Next.js
React
TypeScript
Tailwind
CCC
Nervos CKB
Rust
RISC-V
```

**Main Concepts:**

```text
Cells
Transactions
Lock Scripts
Type Scripts
Cell Deps
Wallets
Ownership
State Transitions
Fraud Prevention
```

**Major Achievement:**

> Built the foundation of a real CKB digital asset dApp with CCC wallet integration and real CKB Testnet transaction construction, while establishing the Rust validator architecture needed to enforce asset ownership rules on-chain.

**Status:**

```text
Frontend                  ████████████████████ 100%
CCC Integration           ████████████████████ 100%
Transaction Foundation    ████████████████████ 100%
Asset Data Model          ████████████████████ 100%

Rust Validator             ███████████████░░░░░  75%
Contract Deployment        ███████░░░░░░░░░░░░░  35%
On-chain Asset Transfer    █████░░░░░░░░░░░░░░░  25%
Fraud Testing              ███░░░░░░░░░░░░░░░░░  15%
```

Week 7 therefore establishes the bridge between **CKB smart-contract development and real decentralized application development**.

