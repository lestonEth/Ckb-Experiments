# Counter Contract

This directory contains the Week 5 CKB Counter Contract.

The contract demonstrates how application state can be represented inside a CKB Cell and how a Rust script can validate a state transition.

## State Format

The Cell stores:

```text
version: u8
counter: u64
```

Total encoded size:

```text
9 bytes
```

## Transition Rule

The contract requires:

```text
output_counter = input_counter + 1
```

Therefore:

```text
0 → 1  ✓
1 → 2  ✓
10 → 11 ✓

1 → 3  ✗
5 → 4  ✗
```

## Rust Implementation

The core implementation is located in:

```text
src/lib.rs
```

The CKB-VM entry point is located in:

```text
src/main.rs
```

## Tests

Run:

```bash
cargo test
```

## Build

Install the RISC-V target:

```bash
make prepare
```

Build the contract:

```bash
make build
```

The contract targets:

```text
riscv64imac-unknown-none-elf
```

## Future Improvements

The current implementation is the first stage of the Week 5 project.

Future work will connect the state validation logic to actual CKB transaction inputs and outputs.

Planned improvements include:

1. Load input Cell data.
2. Load output Cell data.
3. Decode both states.
4. Validate the state transition.
5. Validate the owner.
6. Validate transaction structure.
7. Add transaction-level integration tests.
