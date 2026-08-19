# Week 5 Journal — Stateful Counter Contract

## Week Overview

Week 5 focused on moving from basic CKB script experimentation into **stateful smart contract development using Rust**. The main goal was to understand how a CKB smart contract validates a change in on-chain state rather than directly modifying that state.

The practical project for this week was a **Counter Contract**. The contract maintains a numeric counter inside a CKB cell and validates that each transaction increments the counter by exactly one.

This exercise introduced important concepts around CKB cells, transaction inputs and outputs, state transitions, serialization, Rust-based contract development, and validation logic.

---

## Learning Objectives

The objectives for Week 5 were:

* Understand stateful CKB contracts.
* Understand how contract state is represented inside cells.
* Learn how input and output cells participate in state transitions.
* Implement a CKB contract using Rust.
* Read and validate transaction data from the CKB script environment.
* Ensure that a counter can only increase by one step.
* Understand the difference between application logic and transaction validation.
* Improve familiarity with the CKB Rust development workflow.
* Document the contract structure and validation rules.

---

## Core Concept: Stateful Contracts

A CKB smart contract does not behave like a traditional backend application where a function directly updates a database.

Instead, the state is represented by cells.

A simplified state transition looks like:

```text
Input Cell
Counter = 5
    |
    | Transaction
    v
Output Cell
Counter = 6
```

The contract validates whether this transition is valid.

For the Week 5 counter contract, the required rule is:

```text
output_counter = input_counter + 1
```

Therefore:

```text
5 → 6      Valid
6 → 7      Valid
10 → 11    Valid
5 → 7      Invalid
5 → 5      Invalid
```

The contract is responsible for rejecting transactions that violate this rule.

---

## Project Structure

The Week 5 contract was organized around a dedicated counter contract.

A simplified project structure is:

```text
week5/
├── contracts/
│   └── counter/
│       ├── src/
│       │   └── main.rs
│       ├── Cargo.toml
│       └── README.md
├── README.md
└── Journal.md
```

The important files are:

### `main.rs`

Contains the Rust implementation of the counter validation logic.

### `Cargo.toml`

Defines the Rust package configuration and dependencies required by the contract.

### `README.md`

Documents the purpose of the contract, its state-transition rule, project structure, and development instructions.

### `Journal.md`

Records the learning process, concepts studied, implementation decisions, challenges, and lessons learned during Week 5.

---

## Counter Contract Design

The contract was designed around a simple state machine.

The counter represents the state:

```text
Counter(n)
```

A valid transaction must perform:

```text
Counter(n) → Counter(n + 1)
```

The contract therefore checks two states:

```text
Input State
     |
     v
Counter = N
     |
     | validation
     v
Output State
     |
     v
Counter = N + 1
```

If the output does not represent the next valid state, the transaction must fail.

---

## Validation Rule

The central validation rule is:

```text
output_counter == input_counter + 1
```

This gives the contract a deterministic state transition.

For example:

```text
Input:  0
Output: 1
Result: Valid
```

```text
Input: 1
Output: 2
Result: Valid
```

```text
Input: 2
Output: 4
Result: Invalid
```

```text
Input: 5
Output: 5
Result: Invalid
```

This simple rule demonstrates one of the fundamental principles of blockchain programming: **the contract validates the transition between states rather than trusting the caller to update the state correctly.**

---

## Rust Implementation

The contract implementation was written in Rust.

The implementation is responsible for:

1. Accessing the relevant input cell.
2. Accessing the relevant output cell.
3. Reading the counter state.
4. Comparing the input and output values.
5. Verifying that the output value is exactly one greater.
6. Returning success when the transition is valid.
7. Returning an error when the transition is invalid.

Conceptually, the validation logic is:

```rust
let input_counter = read_input_counter();
let output_counter = read_output_counter();

if output_counter != input_counter + 1 {
    return Err(...);
}

Ok(())
```

Although the actual CKB implementation interacts with the transaction and cell environment, this simplified representation captures the core logic.

---

## Understanding CKB Cells

One of the major learning points this week was understanding that **cells are the fundamental units of state in CKB**.

A cell can contain:

* Capacity
* Lock script
* Type script
* Data

The counter value is stored as cell data.

Conceptually:

```text
+---------------------------+
| CKB Cell                  |
+---------------------------+
| Capacity                  |
+---------------------------+
| Lock Script               |
+---------------------------+
| Type Script               |
+---------------------------+
| Data                      |
|                           |
| Counter = 5               |
+---------------------------+
```

When a transaction consumes this cell, the contract can validate that the newly created output cell contains the expected next state.

---

## Input and Output State

The most important concept learned this week was the relationship between inputs and outputs.

A transaction consumes existing cells as inputs and creates new cells as outputs.

For the counter:

```text
INPUT
Counter = 3

       ↓

TRANSACTION

       ↓

OUTPUT
Counter = 4
```

The contract verifies that:

```text
4 == 3 + 1
```

If the output contains `5`, the contract rejects the transaction because:

```text
5 != 3 + 1
```

This helped reinforce the idea that CKB state changes are represented as **consuming old cells and creating new cells**.

---

## State Transition Model

The counter can be represented as a simple state machine:

```text
       +-------+
       |   0   |
       +-------+
           |
           v
       +-------+
       |   1   |
       +-------+
           |
           v
       +-------+
       |   2   |
       +-------+
           |
           v
       +-------+
       |   3   |
       +-------+
           |
          ...
```

Only a transition to the next state is allowed.

Therefore:

```text
0 → 1 → 2 → 3 → 4 → 5
```

is valid.

But:

```text
0 → 2
```

is invalid.

Likewise:

```text
3 → 3
```

is invalid.

This provided a practical introduction to state-machine thinking in smart contract development.

---

## Transaction Validation

The contract does not need to trust the transaction creator.

Instead, it independently verifies the transaction.

The general process is:

```text
Transaction Submitted
        |
        v
Contract Executes
        |
        v
Read Input State
        |
        v
Read Output State
        |
        v
Compare States
        |
        +------ Invalid ------> Reject
        |
       Valid
        |
        v
     Accept
```

This demonstrates how CKB scripts enforce protocol-level rules.

---

## Valid Transaction Example

Suppose the current cell contains:

```text
Counter = 7
```

A transaction creates an output cell containing:

```text
Counter = 8
```

The contract calculates:

```text
7 + 1 = 8
```

The condition is satisfied.

```text
Result: Valid
```

---

## Invalid Transaction Example

Suppose the input contains:

```text
Counter = 7
```

but the output contains:

```text
Counter = 10
```

The contract expects:

```text
7 + 1 = 8
```

but receives:

```text
10
```

Therefore:

```text
10 != 8
```

The transaction must be rejected.

---

## Another Invalid Case

If the input contains:

```text
Counter = 7
```

and the output contains:

```text
Counter = 7
```

then:

```text
7 != 7 + 1
```

The state did not advance.

The contract therefore rejects the transaction.

---

## Error Handling

Another important part of the implementation was understanding how invalid transactions should be handled.

Instead of silently accepting incorrect state, the contract returns an error.

Conceptually:

```text
Valid state transition
        |
        v
      Success

Invalid state transition
        |
        v
       Error
```

This ensures that invalid state transitions cannot be accepted by the script.

---

## Integer and Overflow Considerations

The counter implementation also introduced an important consideration around integer arithmetic.

If the counter is represented using a fixed-width integer, eventually it could reach its maximum value.

For example, with an unsigned 8-bit integer:

```text
255 + 1
```

cannot be represented using the same type.

For a production contract, arithmetic should therefore be handled carefully to avoid overflow.

The general principle learned was:

> Smart contract arithmetic must be deterministic and must explicitly handle invalid or boundary conditions.

---

## Challenges Encountered

Several areas required additional attention during this week.

### 1. Understanding CKB's State Model

Initially, it was easy to think about the counter as if it were a normal variable that the contract could simply modify.

The CKB model is different.

The contract validates a transition:

```text
Old Cell → New Cell
```

rather than directly modifying persistent state.

---

### 2. Understanding Transaction Context

The contract needs to work with information provided by the CKB transaction environment.

This required understanding how scripts access:

* Input cells
* Output cells
* Cell data
* Script context

This was an important step toward understanding real CKB contract development.

---

### 3. Separating State From Validation Logic

Another learning point was separating the counter's state representation from the rules that validate it.

The counter is simply data:

```text
Counter = N
```

The contract provides the rule:

```text
N_output = N_input + 1
```

This separation makes the contract easier to reason about.

---

### 4. Rust and CKB-VM Constraints

Writing Rust for a CKB contract is different from writing a conventional Rust application.

The contract executes within the CKB-VM environment, meaning the implementation must follow the constraints of the target environment and available CKB libraries.

This helped reinforce the importance of understanding the runtime in which smart contract code executes.

---

## Testing Strategy

Testing the counter contract should focus on both valid and invalid state transitions.

### Valid Cases

| Input | Output | Expected |
| ----: | -----: | -------- |
|     0 |      1 | Valid    |
|     1 |      2 | Valid    |
|     2 |      3 | Valid    |
|    10 |     11 | Valid    |

### Invalid Cases

| Input | Output | Expected |
| ----: | -----: | -------- |
|     0 |      0 | Invalid  |
|     0 |      2 | Invalid  |
|     5 |      7 | Invalid  |
|    10 |      9 | Invalid  |

The key property being tested is:

```text
output = input + 1
```

Any transaction that violates this property should fail validation.

---

## Security Considerations

Even though the counter contract is simple, it introduced several important smart-contract security principles.

### State Integrity

The contract must ensure that the output state cannot arbitrarily change.

### Deterministic Execution

The same transaction should produce the same validation result.

### Input/Output Consistency

The contract must validate that the correct input state is being transformed into the correct output state.

### Arithmetic Safety

Counter arithmetic must account for integer limits and overflow conditions.

### Script Binding

The state should be associated with the correct script so that an attacker cannot simply create unrelated cells containing arbitrary counter values and treat them as legitimate contract state.

---

## What I Learned

By the end of Week 5, I had a better understanding of how CKB smart contracts manage state.

The most important lessons were:

1. **CKB state is represented by cells.**
2. **Transactions consume input cells and create output cells.**
3. **Contracts validate state transitions.**
4. **Rust can be used to implement CKB-VM scripts.**
5. **A stateful contract can enforce deterministic rules about how state changes.**
6. **Transaction validation should not trust user-provided state.**
7. **Simple state machines are useful for understanding blockchain application logic.**
8. **Smart contract arithmetic and boundary conditions require careful handling.**

---

## Reflection

Week 5 was a significant progression from the earlier stages of the learning journey.

Previously, my focus was primarily on understanding CKB scripts and the development environment. With the counter contract, I started applying those concepts to a practical stateful use case.

The counter is intentionally simple, but it demonstrates an important blockchain pattern:

```text
Current State
     ↓
Transaction
     ↓
Proposed New State
     ↓
Contract Validation
     ↓
Accept / Reject
```

This helped me understand that CKB smart contracts are fundamentally about **verifying valid state transitions**.

The exercise also gave me a clearer foundation for more complex contracts where the state may contain multiple fields and the transition rules may involve ownership, authorization, balances, or other application-specific conditions.

---

## Week 5 Outcome

By completing Week 5, I established the foundation for stateful CKB contract development.

The completed work includes:

* Counter contract project structure.
* Rust-based contract implementation.
* Counter state representation.
* One-step increment validation.
* Documentation and README.
* Understanding of CKB input/output state transitions.
* Identification of valid and invalid transaction cases.
* Initial consideration of arithmetic and security edge cases.

The central rule implemented during the week was:

```text
Input Counter + 1 = Output Counter
```

This simple rule became the foundation for understanding how more sophisticated CKB contracts can enforce application-specific state transitions.

---

## Next Steps

For the next stage of the learning journey, I plan to build on the counter contract by exploring more complex state and validation logic.

Potential areas include:

* Multiple pieces of state.
* More complex transaction validation.
* Script arguments.
* Cell dependencies.
* Lock and type script interaction.
* More comprehensive contract testing.
* Error handling and failure cases.
* Deployment and execution in a CKB development environment.

The goal is to progressively move from a simple one-step state transition toward contracts that model more realistic decentralized applications.

---

## Conclusion

Week 5 provided a practical introduction to **stateful smart contract development on CKB using Rust**. The counter contract demonstrated how a CKB script can enforce a strict transition between an input state and an output state.

The key concept I am carrying forward is that CKB contracts do not simply "change" blockchain state. Instead, they **validate that a transaction transforms one valid cell state into another valid state according to predefined rules**.

The counter contract is a small implementation, but it establishes the core foundation needed to understand and build more advanced CKB smart contracts.
