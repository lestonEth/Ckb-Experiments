# Week 5 — CKB Cell Counter Contract

## Nervos CKB Developer Learning Journey

This project is the Week 5 practical milestone of my Nervos CKB developer learning journey.

The objective is to move beyond basic script execution and study how **state can be represented inside CKB Cells and validated across transactions**.

The project implements a simple Counter Cell whose state can only transition by one increment at a time.

```text
Counter = 0
    ↓
Counter = 1
    ↓
Counter = 2
    ↓
Counter = 3
```

A transaction attempting to skip a state is rejected by the validation logic.

```text
Counter = 1
    ↓
Counter = 3

INVALID
```

---

## Learning Objective

The primary objective of Week 5 is to understand the relationship between:

* Cells
* Cell data
* Transactions
* State transitions
* Rust scripts
* CKB-VM
* Input Cells
* Output Cells
* Contract validation

Unlike account-based blockchain architectures, CKB state is represented through Cells.

A Cell is consumed as a transaction input and new Cells are created as transaction outputs.

Therefore, updating application state can be represented as:

```text
Existing Cell
     │
     │ consumed
     ▼
  Transaction
     │
     │ creates
     ▼
Updated Cell
```

The Counter Contract uses this model to demonstrate a controlled state transition.

---

# Counter Data Model

The Counter Cell stores 9 bytes of data.

```text
Byte 0
┌──────────────┐
│   version    │
└──────────────┘

Bytes 1 - 8
┌──────────────────────────┐
│        counter           │
│          u64             │
└──────────────────────────┘
```

The layout is:

```text
version : u8
counter : u64
```

The counter is encoded using little-endian byte order.

Example:

```rust
CounterData {
    version: 1,
    counter: 42
}
```

is serialized into a 9-byte array.

---

# State Transition Rule

The central rule of the contract is:

```text
output_counter = input_counter + 1
```

Valid:

```text
0 → 1
1 → 2
2 → 3
10 → 11
```

Invalid:

```text
0 → 2
5 → 10
10 → 9
```

The contract also protects against integer overflow.

For example:

```text
u64::MAX → 0
```

is rejected.

---

# Contract Structure

```text
week-05-cell-counter/
│
├── Cargo.toml
├── Makefile
├── README.md
│
├── contracts/
│   └── counter/
│       ├── Cargo.toml
│       ├── Makefile
│       └── src/
│           ├── lib.rs
│           └── main.rs
│
├── tests/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── tests.rs
│
└── journal/
    └── week-05.md
```

---

# Main Components

## `lib.rs`

Contains the reusable counter implementation:

* Counter data structure
* Serialization
* Deserialization
* Version validation
* Increment validation
* Overflow protection
* Unit tests

The main validation function is:

```rust
pub fn validate_increment(
    input_counter: u64,
    output_counter: u64,
) -> Result<(), CounterError>
```

---

## `main.rs`

Contains the CKB contract entry point.

The contract is compiled for:

```text
riscv64imac-unknown-none-elf
```

and executed inside the CKB-VM environment.

The current Week 5 implementation verifies that the contract can load its script successfully.

The next stage will connect the entry point to actual input/output Cell data.

---

# Error Handling

The contract currently defines:

```rust
pub enum CounterError {
    InvalidDataLength,
    InvalidVersion,
    InvalidTransition,
    CounterOverflow,
}
```

### InvalidDataLength

Returned when Cell data does not contain exactly the expected number of bytes.

### InvalidVersion

Returned when the Cell contains an unsupported data version.

### InvalidTransition

Returned when the output counter does not equal the input counter plus one.

### CounterOverflow

Returned when incrementing the maximum `u64` value would overflow.

---

# Testing

The contract includes tests for:

* Initial counter creation
* Valid increment
* Multiple valid increments
* Skipped increments
* Counter decrements
* Integer overflow
* Data encoding
* Data decoding
* Invalid data length
* Invalid version

Run:

```bash
cargo test --workspace
```

Expected result:

```text
test result: ok
```

---

# Building the Contract

Before building the CKB contract, install the RISC-V target:

```bash
make prepare
```

Then build:

```bash
make build
```

The target architecture is:

```text
riscv64imac-unknown-none-elf
```

---

# Formatting

Run:

```bash
make fmt
```

---

# Static Checking

Run:

```bash
make check
```

---

# Cleaning Build Artifacts

Run:

```bash
make clean
```

---

# Development Progress

### Completed

* [x] Created Week 5 Counter project
* [x] Defined Counter Cell data format
* [x] Implemented counter serialization
* [x] Implemented counter deserialization
* [x] Added version validation
* [x] Implemented one-step increment validation
* [x] Added overflow protection
* [x] Added unit tests
* [x] Created CKB-VM entry point
* [x] Added RISC-V build configuration

### Next

* [ ] Load actual input Cell data
* [ ] Load actual output Cell data
* [ ] Compare input and output Cell states
* [ ] Reject invalid on-chain state transitions
* [ ] Add transaction-level tests
* [ ] Add owner authorization
* [ ] Test the contract against a local CKB environment
* [ ] Document transaction construction

---

# Learning Outcome

The Week 5 implementation demonstrates an important CKB development concept:

> Application state can be represented as Cell data and state changes can be enforced by scripts validating transitions between input and output Cells.

The Counter Contract provides a small but concrete example of this model.

The project will be extended during the week to move the validation from pure Rust unit tests into actual CKB Cell and transaction validation.

---

# Technologies

* Rust
* Nervos CKB
* CKB-VM
* RISC-V
* `ckb-std`
* Cargo
* Make
* Unit Testing

---

# Week 5 Milestone

At the end of this stage, the project demonstrates a working foundation for a stateful CKB contract.

The next implementation milestone is to make the contract inspect actual transaction inputs and outputs:

```text
Input Cell
    │
    │ counter = N
    ▼
Counter Script
    │
    │ validate
    ▼
Output Cell
    │
    │ counter = N + 1
    ▼
Transaction accepted
```

Any transaction that violates the state transition rule should cause the script to return an error.
