# Week 6 Journal — CKB Transactions, Cell Validation, and Fraudulent Transactions

## Overview

Week 6 focused on understanding **CKB transactions at a deeper level** and translating that knowledge into a practical Rust transaction validator.

In the previous weeks, I worked with basic CKB Scripts and a counter contract. This week shifted the focus from writing a simple state-changing contract to understanding how CKB transactions consume existing Cells, create new Cells, reference dependencies, carry witnesses, and enforce state transitions.

The main objective was to understand how transaction validation works and how invalid or fraudulent transaction patterns can be detected.

The implementation was created under:

```text
contracts/
└── transaction-validator/
```

---

## Week 6 Objectives

The objectives for this week were:

1. Understand the complete CKB transaction structure.
2. Understand how Cells are consumed and created.
3. Learn how `OutPoint` identifies a specific Cell.
4. Understand `cell_deps`.
5. Understand `header_deps`.
6. Understand `CellInput` and `since`.
7. Understand `CellOutput` and `outputs_data`.
8. Understand transaction witnesses.
9. Implement transaction validation in Rust.
10. Detect duplicate inputs.
11. Simulate double-spend scenarios.
12. Validate transaction capacity.
13. Validate counter state transitions.
14. Handle integer overflow safely.
15. Explore how transaction information can be accessed from CKB-VM.

---

# Day 1 — Understanding the CKB Transaction Model

I started the week by studying the CKB transaction structure.

A CKB transaction is fundamentally different from a traditional account-based transaction model.

Instead of saying:

```text
Alice -> Bob
10 CKB
```

a CKB transaction consumes existing Cells and creates new Cells.

The simplified model is:

```text
Existing Cells
      │
      ▼
   Inputs
      │
      ▼
 Transaction
      │
      ▼
   Outputs
      │
      ▼
New Cells
```

I learned that the main transaction fields are:

```text
version
cell_deps
header_deps
inputs
outputs
outputs_data
witnesses
```

This was important because my earlier transaction model only represented inputs and outputs.

I updated the Week 6 design to represent the complete transaction structure.

---

# Day 2 — Understanding OutPoint and CellInput

The second stage focused on transaction inputs.

A `CellInput` references a previously created Cell using an `OutPoint`.

The structure is:

```text
CellInput
│
├── previous_output
│   ├── transaction hash
│   └── output index
│
└── since
```

The `OutPoint` is important because it uniquely identifies a Cell.

For example:

```text
Transaction Hash:
0xABC...

Output Index:
0
```

identifies:

```text
0xABC... : Cell 0
```

I implemented:

```rust
pub struct OutPoint {
    pub tx_hash: [u8; 32],
    pub index: u32,
}
```

and:

```rust
pub struct CellInput {
    pub previous_output: OutPoint,
    pub since: u64,
}
```

This gave the transaction model a much closer relationship to the real CKB structure.

---

# Day 3 — Cell Dependencies and Header Dependencies

I then studied transaction dependencies.

## Cell Dependencies

`cell_deps` allow a transaction's Scripts to access referenced Cells without consuming them.

A dependency contains:

```text
CellDep
│
├── out_point
└── dep_type
```

The dependency type can be:

```text
code
```

or:

```text
dep_group
```

I learned that this is particularly important for smart contract execution because executable Script code can be stored in a Cell and referenced by transactions.

## Header Dependencies

I also studied `header_deps`.

These contain block header hashes that Scripts can access during execution.

The important concept I learned was **determinism**.

A Script should produce the same result across different nodes. Therefore, the transaction cannot arbitrarily depend on unavailable or inconsistent block information.

---

# Day 4 — Understanding Cells and Outputs

The next part of the week focused on Cells.

A CKB Cell contains:

```text
Cell
│
├── capacity
├── lock
├── type
└── data
```

I implemented a Rust representation:

```rust
pub struct Cell {
    pub capacity: u64,
    pub lock: Script,
    pub type_script: Option<Script>,
    pub data: Vec<u8>,
}
```

This helped connect the transaction model to the actual state model of CKB.

I also learned that transaction outputs and output data are represented separately:

```text
outputs[0]      <-> outputs_data[0]

outputs[1]      <-> outputs_data[1]
```

This means the number of output Cells must correspond to the number of output data entries.

I added validation for this relationship.

---

# Day 5 — Transaction Validation

After understanding the transaction structure, I began implementing the validator.

The validator performs several checks.

### Transaction Structure

The transaction must contain:

```text
At least one input
At least one output
```

### Version

The transaction version must be supported.

### Input Validation

Inputs must contain valid OutPoints.

### Duplicate Input Detection

The same Cell cannot be referenced twice in the same transaction.

### Capacity Validation

The total output capacity cannot exceed the total input capacity.

### Output Data Validation

Every output must have corresponding output data.

The main validation flow became:

```text
Transaction
     │
     ▼
Validate Structure
     │
     ▼
Validate Version
     │
     ▼
Validate Inputs
     │
     ├── Missing input?
     └── Duplicate input?
     │
     ▼
Validate Dependencies
     │
     ▼
Validate Outputs
     │
     ├── Capacity
     └── Output data
     │
     ▼
VALID
```

---

# Day 6 — Fraudulent Transactions and Double Spending

One of the most important parts of this week was understanding fraudulent transaction scenarios.

I created tests for invalid transactions rather than only testing successful transactions.

## Duplicate Input

Example:

```text
Input 0 -> Cell A
Input 1 -> Cell A
```

The same Cell is referenced twice.

The validator returns:

```text
DuplicateInput
```

## Capacity Inflation

Another test attempts to create more capacity than the transaction consumes.

```text
Input:
1000

Output:
1500
```

The validator rejects this with:

```text
InsufficientCapacity
```

## Missing Input

A transaction referencing an invalid or zero OutPoint is rejected.

```text
tx_hash = [0u8; 32]
```

returns:

```text
MissingInput
```

## Invalid State Transition

The counter validation from Week 5 was also integrated into the transaction validator.

Valid:

```text
10 -> 11
```

Invalid:

```text
10 -> 15
```

Invalid:

```text
10 -> 9
```

---

# Day 7 — Understanding Cross-Transaction Double Spending

An important lesson from this week was distinguishing between **duplicate inputs inside one transaction** and **conflicting transactions**.

For example:

```text
Transaction A
Input -> Cell A
```

and:

```text
Transaction B
Input -> Cell A
```

represent two separate transactions attempting to consume the same Cell.

A Script does not simply scan the mempool and decide whether another pending transaction exists.

The CKB node and consensus rules handle transaction conflicts.

The contract can, however, detect:

```text
Transaction A

Input 0 -> Cell A
Input 1 -> Cell A
```

inside the same transaction.

This distinction changed how I thought about smart contract security.

Not every validation rule belongs inside the Script. Some rules are enforced by the CKB transaction validation and consensus layer.

---

# Witnesses

I also studied transaction witnesses.

Witnesses provide additional data that Scripts can use during validation.

The conventional CKB `WitnessArgs` structure contains:

```text
WitnessArgs
│
├── lock
├── input_type
└── output_type
```

This is particularly important for lock scripts and type scripts.

For example, signature information can be included in the witness data and then verified by a Lock Script.

For Week 6, I created a simplified Rust representation:

```rust
pub struct WitnessArgs {
    pub lock: Option<Vec<u8>>,
    pub input_type: Option<Vec<u8>>,
    pub output_type: Option<Vec<u8>>,
}
```

I intentionally kept this simplified because full Molecule serialization and decoding will be handled in a later stage.

---

# CKB Runtime Exploration

Towards the end of the week, I started connecting the educational transaction model to actual CKB-VM execution.

I studied the CKB syscall model and how Scripts can access:

```text
Transaction
Inputs
Outputs
Cell data
Witnesses
Transaction hash
```

This led to the creation of a runtime abstraction:

```text
runtime.rs
```

The intended execution model is:

```text
CKB Node
   │
   ▼
CKB-VM
   │
   ▼
Script
   │
   ├── Load transaction
   ├── Load input Cells
   ├── Load output Cells
   ├── Load Cell data
   ├── Load witnesses
   └── Validate state
```

This was an important step toward moving from simulated Rust objects to actual CKB transaction execution.

---

# Testing

I created tests covering both valid and invalid transactions.

The test suite includes:

```text
Valid transaction
Invalid transaction version
Duplicate inputs
Missing input
Output capacity exceeding input capacity
Output data mismatch
Duplicate Cell dependencies
Valid counter increment
Counter skip
Counter decrement
Counter overflow
```

The overall testing strategy became:

```text
Valid Case
    │
    └── Must PASS

Invalid Case
    │
    └── Must FAIL with expected error
```

This made the validator much more useful as a learning project because every validation rule had an associated test.

---

# Key Technical Lessons

## 1. CKB is Cell-based

The biggest lesson this week was understanding that CKB transactions operate around Cells.

A transaction consumes existing Cells and creates new Cells.

---

## 2. OutPoint identifies a Cell

An OutPoint combines:

```text
transaction hash
+
output index
```

to identify a specific Cell.

---

## 3. Transaction validation is layered

Not every validation rule belongs to a Script.

Some rules are enforced by:

```text
CKB transaction validation
```

while others are enforced by:

```text
Lock Scripts
Type Scripts
```

and others by:

```text
CKB consensus
```

Understanding this separation is essential when designing CKB smart contracts.

---

## 4. Capacity cannot simply be created

The transaction must preserve the capacity conservation rule:

```text
Input Capacity >= Output Capacity
```

with the difference accounting for transaction fees.

---

## 5. State is stored in Cells

The counter contract from Week 5 helped demonstrate this.

For example:

```text
Cell Data

10
```

can be consumed and replaced with:

```text
Cell Data

11
```

The Type Script can enforce that the transition is valid.

---

## 6. Overflow must be handled explicitly

Rust's checked arithmetic is useful when implementing financial and state-transition logic.

Instead of:

```rust
input + 1
```

I used:

```rust
input.checked_add(1)
```

This allows the contract to explicitly reject overflow.

---

# Challenges Encountered

The biggest challenge this week was moving from a simplified transaction representation to the actual CKB transaction model.

The initial model was:

```text
Transaction
├── Inputs
└── Outputs
```

After studying the CKB documentation, the model became:

```text
Transaction
├── version
├── cell_deps
├── header_deps
├── inputs
├── outputs
├── outputs_data
└── witnesses
```

Another challenge was understanding the difference between transaction-level validation and Script-level validation.

It became clear that trying to make a single Script responsible for every possible transaction security rule would be incorrect.

---

# Week 6 Architecture

The resulting project architecture is:

```text
transaction-validator/
│
├── transaction.rs
│   └── Transaction structures
│
├── cell.rs
│   └── Cell and Script structures
│
├── validator.rs
│   └── Validation rules
│
├── errors.rs
│   └── Validation errors
│
├── witness.rs
│   └── Witness representation
│
├── ckb_transaction.rs
│   └── CKB transaction abstraction
│
├── runtime.rs
│   └── CKB-VM transaction access
│
├── main.rs
│   └── Script entry point
│
└── transaction_tests.rs
    └── Validation tests
```

---

# Security Perspective

The fraudulent transaction tests helped me understand that smart contract security is largely about **state integrity**.

The validator must prevent invalid transitions such as:

```text
Valid Cell
    │
    ▼
Invalid transaction
    │
    X
REJECT
```

Examples include:

```text
Duplicate Cell consumption
Capacity inflation
Invalid Cell references
Invalid counter transitions
Integer overflow
Malformed transaction structure
```

The goal is not simply to reject malicious transactions, but to make sure that every accepted transaction produces a valid state transition.

---

# What I Built This Week

By the end of Week 6, I had created:

```text
✓ Full transaction data model
✓ Cell and Script structures
✓ OutPoint representation
✓ CellInput representation
✓ CellDep representation
✓ Transaction validator
✓ Capacity validation
✓ Duplicate input detection
✓ Missing input detection
✓ Output data validation
✓ Counter transition validation
✓ Overflow protection
✓ Witness representation
✓ Runtime transaction abstraction
✓ Fraudulent transaction scenarios
✓ Unit tests
✓ CKB-VM-oriented entry point
```

---

# Reflection

Week 6 was a major step forward from simply writing a CKB Script.

Previously, I was mainly focused on getting a contract to compile and understanding basic Cell state.

This week forced me to think about what actually happens when a transaction moves through the CKB system.

I learned that a transaction is effectively a state transition:

```text
Old State
   │
   │ consume Cells
   ▼
Transaction
   │
   │ validate
   ▼
New State
```

For a transaction to be accepted, the transition must satisfy the rules defined by the CKB protocol and the Scripts attached to the Cells.

The fraudulent transaction exercises were particularly useful because they changed my perspective from:

> "How do I make the contract work?"

to:

> "How can I prove that an invalid state transition cannot be accepted?"

That mindset is important for smart contract development.

---

# Next Week

The next stage will move deeper into actual CKB Script execution.

The planned areas are:

```text
Week 7
│
├── CKB-VM execution
├── Script groups
├── Input / output Cell loading
├── CKB syscalls
├── Molecule serialization
├── Real transaction fixtures
├── WitnessArgs decoding
├── Type Script execution
└── Integration testing
```

The goal will be to move beyond simulated Rust transaction objects and start validating **real CKB transaction data inside the CKB-VM environment**.

---

# Final Week 6 Summary

Week 6 focused on understanding the relationship between:

```text
Transactions
      +
Cells
      +
Scripts
      +
Witnesses
      +
CKB-VM
      +
Consensus
```

The most important lesson was that CKB's Cell model makes transaction validation fundamentally about **verifying state transitions**.

The Week 6 transaction validator provides a practical foundation for continuing into deeper CKB development, where the next step is to connect these validation concepts to actual serialized transactions and CKB-VM execution.

