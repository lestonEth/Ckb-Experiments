# CKB Transaction Validator

## Week 6 — CKB Developer Learning Journey

This project explores **Nervos CKB transaction structure, Cell consumption, transaction validation, state transitions, witnesses, and fraudulent transaction scenarios** using Rust.

The implementation starts with a simplified transaction model and progressively introduces concepts that map to the actual CKB transaction structure and CKB-VM execution environment.

---

## Objectives

By the end of Week 6, this project demonstrates:

* CKB transaction structure
* Cell inputs and outputs
* `OutPoint` references
* Transaction versioning
* Cell dependencies
* Header dependencies
* Input `since` conditions
* Output capacity validation
* `outputs_data` validation
* Witness handling
* Duplicate input detection
* Simulated double-spend detection
* Missing input detection
* Counter state transitions
* Integer overflow protection
* Fraudulent transaction simulations
* CKB runtime transaction loading
* CKB-VM-oriented validation

---

## CKB Transaction Structure

A CKB transaction consumes existing Cells and creates new Cells.

```text
Transaction
│
├── version
│
├── cell_deps[]
│   └── OutPoint
│       ├── tx_hash
│       └── index
│
├── header_deps[]
│   └── block header hash
│
├── inputs[]
│   └── CellInput
│       ├── previous_output
│       │   ├── tx_hash
│       │   └── index
│       └── since
│
├── outputs[]
│   └── CellOutput
│       ├── capacity
│       ├── lock
│       └── type
│
├── outputs_data[]
│   └── Cell state
│
└── witnesses[]
    └── WitnessArgs / custom proof
```

The transaction essentially performs:

```text
Previous Cells
      │
      │ consumed
      ▼
┌───────────────┐
│  Transaction   │
└───────────────┘
      │
      │ creates
      ▼
New Cells
```

---

## Project Structure

```text
transaction-validator/
├── Cargo.toml
├── Makefile
├── README.md
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── errors.rs
│   ├── transaction.rs
│   ├── cell.rs
│   ├── validator.rs
│   ├── ckb_transaction.rs
│   ├── runtime.rs
│   ├── witness.rs
│   └── transaction_tests.rs
└── tests/
    └── fraudulent_transactions.md
```

---

## Transaction Components

### 1. Version

The transaction contains a `version` field.

For the current transaction format, the expected version is:

```text
version = 0
```

The validator rejects unsupported transaction versions.

---

### 2. Cell Dependencies

`cell_deps` references Cells that the transaction depends on but does not consume.

A dependency is represented by an `OutPoint`:

```text
CellDep
│
└── OutPoint
    ├── transaction hash
    └── output index
```

A dependency can be interpreted as either:

```text
Code
```

or:

```text
DepGroup
```

A code dependency allows a Script to access executable code, while a dep group can reference multiple dependencies through a single Cell.

---

### 3. Header Dependencies

`header_deps` contains hashes of block headers that Scripts may access.

These dependencies must already exist on-chain so that Script execution remains deterministic.

```text
Transaction
     │
     └── header_deps[]
             │
             ▼
       Block Header Hash
```

---

## Inputs

Each transaction input references an existing Cell.

```text
CellInput
│
├── previous_output
│   ├── tx_hash
│   └── index
│
└── since
```

The `previous_output` identifies the exact Cell being consumed.

For example:

```text
Transaction:
0xABC...

Output index:
0
```

means:

```text
0xABC... : Cell 0
```

---

## `since`

The `since` field allows an input to have a time or block-related restriction.

It can represent conditions based on:

* Block number
* Epoch
* Timestamp

For example:

```text
Input
│
└── since
     │
     └── Cannot be spent before condition is satisfied
```

If an input's `since` requirement has not been satisfied, the transaction is invalid.

---

## Outputs

Outputs represent newly created Cells.

```text
CellOutput
│
├── capacity
├── lock
└── type
```

A Cell therefore contains both ownership rules and state.

```text
Cell
│
├── capacity
├── lock script
├── type script
└── data
```

---

## Capacity Validation

The transaction must not create more capacity than it consumes.

Valid:

```text
Input capacity:
1000

Output capacity:
900

Result:
VALID
```

Invalid:

```text
Input capacity:
1000

Output capacity:
1500

Result:
REJECTED
```

The validator also uses checked arithmetic to prevent integer overflow.

---

## `outputs_data`

`outputs_data` stores the data associated with each output Cell.

The relationship is positional:

```text
outputs[0]      <-> outputs_data[0]

outputs[1]      <-> outputs_data[1]

outputs[2]      <-> outputs_data[2]
```

Therefore:

```text
outputs.len()
==
outputs_data.len()
```

must hold.

For example:

```text
outputs:
    [Cell 0, Cell 1]

outputs_data:
    [Data 0, Data 1]
```

is valid.

But:

```text
outputs:
    [Cell 0, Cell 1]

outputs_data:
    [Data 0]
```

is invalid.

---

## Witnesses

Witnesses provide additional data required by Scripts.

A common CKB convention is `WitnessArgs`:

```text
WitnessArgs
│
├── lock
├── input_type
└── output_type
```

The `lock` field can contain signature-related information.

The `input_type` and `output_type` fields can contain data used by Type Scripts.

Conceptually:

```text
Transaction
     │
     └── witnesses[]
             │
             ▼
       Script validation
```

This project currently models witnesses as byte vectors and provides a simplified `WitnessArgs` representation. Full Molecule decoding can be introduced in a later stage.

---

## Validation Rules

The validator performs several checks.

### 1. Transaction Structure

The transaction must contain:

```text
At least one input
At least one output
```

---

### 2. Transaction Version

Unsupported transaction versions are rejected.

```text
version = 0
```

is accepted.

An unsupported version such as:

```text
version = 1
```

is rejected by the current validator.

---

### 3. Duplicate Inputs

The same Cell cannot appear twice within the same transaction.

Invalid:

```text
Input 0 -> TX_A:0
Input 1 -> TX_A:0
```

Result:

```text
REJECTED
DuplicateInput
```

---

### 4. Missing Input

An input containing an invalid or zero `OutPoint` is rejected by the educational validator.

```text
Input
 │
 └── tx_hash = 0x0000...
```

Result:

```text
REJECTED
MissingInput
```

In a real CKB transaction, the node determines whether an `OutPoint` resolves to a valid live Cell.

---

### 5. Cell Dependency Validation

The validator checks that:

* Dependency OutPoints are valid.
* The same dependency is not unnecessarily duplicated.

Example:

```text
CellDep 0 -> TX_A:0
CellDep 1 -> TX_A:0
```

Result:

```text
REJECTED
DuplicateCellDep
```

---

### 6. Output Data Validation

Every output must have corresponding output data.

```text
outputs.len()
==
outputs_data.len()
```

Otherwise:

```text
REJECTED
OutputsDataMismatch
```

---

### 7. Capacity Validation

The validator calculates the total output capacity using checked arithmetic.

```text
Input Capacity >= Output Capacity
```

If:

```text
Output Capacity > Input Capacity
```

the transaction is rejected.

---

## Counter State Validation

Week 5 introduced a simple counter contract.

Week 6 extends that concept into transaction validation.

The counter must increase exactly once.

Valid:

```text
Input state:
10

Output state:
11
```

Invalid:

```text
Input state:
10

Output state:
15
```

Also invalid:

```text
Input state:
10

Output state:
9
```

The expected transition is:

```text
output_counter = input_counter + 1
```

---

## Counter Overflow

The validator uses checked arithmetic.

For example:

```text
Input:

u64::MAX
```

Attempting:

```text
u64::MAX + 1
```

would overflow.

Instead of allowing wrapping:

```text
u64::MAX + 1 -> 0
```

the validator returns:

```text
CounterOverflow
```

---

# Fraudulent Transaction Scenarios

This project deliberately includes invalid transactions to understand how transaction validation protects Cell state.

These scenarios are simulations for educational and security-testing purposes.

---

## 1. Duplicate Input / Double Spend Simulation

```text
             Cell A
             /   \
            /     \
     Input 0       Input 1
          \         /
           \       /
          Transaction
```

If both inputs reference the same Cell:

```text
Input 0 -> TX_A:0
Input 1 -> TX_A:0
```

the validator rejects the transaction.

```text
DuplicateInput
```

---

## 2. Capacity Inflation

An attacker attempts to create more capacity than was consumed.

```text
Input:

1000 shannons

Output:

1500 shannons
```

Result:

```text
REJECTED
InsufficientCapacity
```

---

## 3. Missing Cell

The transaction references an invalid Cell:

```text
Input
 │
 ▼
Unknown OutPoint
 │
 ▼
REJECT
```

Result:

```text
MissingInput
```

---

## 4. Invalid State Transition

An attacker attempts to modify the counter incorrectly.

```text
Input counter:
10

Output counter:
100
```

Expected:

```text
11
```

Result:

```text
REJECTED
InvalidCounterTransition
```

---

## 5. Counter Overflow

```text
Input counter:
u64::MAX

Output counter:
0
```

Result:

```text
REJECTED
CounterOverflow
```

---

## Important Double-Spend Distinction

A CKB Script does not independently scan the mempool to determine whether another separate transaction is attempting to consume the same Cell.

There are two different scenarios.

### Duplicate input inside one transaction

```text
Transaction A

Input 0 -> Cell A
Input 1 -> Cell A
```

Our validator can detect this.

### Two separate conflicting transactions

```text
              Cell A
              /    \
             /      \
            ▼        ▼
      Transaction A  Transaction B
```

Both transactions may reference the same Cell.

The conflict between separate transactions is handled by CKB transaction validation and consensus rather than by this Script alone.

Therefore, this project calls the duplicate-input test a **double-spend simulation**, while actual cross-transaction double-spend conflicts belong to the CKB node and consensus layer.

---

# CKB Runtime

The project also introduces a runtime-oriented validation layer.

The CKB-VM environment allows Scripts to access transaction and Cell information through CKB syscalls.

The runtime layer is intended to load information such as:

```text
Transaction hash
Input Cells
Output Cells
Input capacity
Output capacity
Cell data
Witnesses
```

Conceptually:

```text
CKB Transaction
       │
       ▼
   CKB-VM
       │
       ▼
   Script
       │
       ├── Load Inputs
       ├── Load Outputs
       ├── Load Cell Data
       ├── Load Witnesses
       └── Validate State
```

This is the bridge between the simplified Rust transaction model and actual CKB Script execution.

---

# Testing

Run the Rust unit tests:

```bash
cargo test
```

For detailed output:

```bash
cargo test -- --nocapture
```

Run formatting:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy
```

Run the compiler checks:

```bash
cargo check
```

---

# Building for CKB-VM

Install the RISC-V target:

```bash
rustup target add riscv64imac-unknown-none-elf
```

or:

```bash
make prepare
```

Build the contract:

```bash
make build
```

Clean the project:

```bash
make clean
```

---

# Expected Test Coverage

The Week 6 test suite should cover:

```text
Transaction Structure
        │
        ├── Empty Inputs
        ├── Empty Outputs
        └── Invalid Version
        │
        ▼
Input Validation
        │
        ├── Missing Input
        └── Duplicate Input
        │
        ▼
Dependency Validation
        │
        ├── Invalid CellDep
        └── Duplicate CellDep
        │
        ▼
Output Validation
        │
        ├── Capacity Overflow
        ├── Insufficient Capacity
        └── outputs_data mismatch
        │
        ▼
State Validation
        │
        ├── Valid Counter Increment
        ├── Counter Skip
        ├── Counter Decrement
        └── Counter Overflow
        │
        ▼
Witness / Runtime Validation
```

---

# Learning Outcomes

By completing Week 6, the main goal is to understand that a CKB transaction is not simply a transfer of an amount.

A CKB transaction consumes existing Cells and creates new Cells.

The transaction contains:

```text
Version
Cell Dependencies
Header Dependencies
Inputs
Outputs
Output Data
Witnesses
```

The validator must therefore reason about both the **transaction structure** and the **state represented by Cells**.

The key concept for this week is:

```text
Transaction
     │
     ├── consumes existing Cells
     │
     ├── validates dependencies
     │
     ├── validates witnesses
     │
     ├── validates capacity
     │
     ├── validates state transitions
     │
     └── creates new Cells
```

This provides the foundation for the next stage of the learning journey: moving from simulated transaction objects to **real CKB transactions, Cell loading, Molecule serialization, Script groups, and integration tests executed through CKB-VM**.

