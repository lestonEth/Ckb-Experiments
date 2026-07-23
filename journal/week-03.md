# CKBuilder Program

# Week 3 Journal

## Project: Multi-Role Owner Lock Script

**Prepared by:** Jimleston Osoi
**Program:** CKBuilder Program – Nervos CKB Developer Track
**Week:** 3

---

# Introduction

During Week 3 of the CKBuilder Program, I focused on understanding how Lock Scripts control ownership and authorization on Nervos CKB.

In Week 2, I developed a Type Script that validated the structure and lifecycle of a profile Cell. For Week 3, I extended my knowledge by creating a more advanced Lock Script that determines who is authorized to consume a Cell.

The project implements a multi-role authorization system with a primary owner and a recovery owner. It also introduces structured script arguments, structured witness data, versioning, action codes, nonce validation, binary parsing, and detailed error handling.

This project helped me understand how CKB separates ownership rules from application-specific validation rules.

---

# Objectives

The main objectives for Week 3 were to:

* Understand the purpose of Lock Scripts.
* Learn the difference between Lock Scripts and Type Scripts.
* Read configuration data from script arguments.
* Read authorization data from transaction witnesses.
* Implement primary-owner and recovery-owner authorization.
* Parse structured binary data safely.
* Introduce contract versioning.
* Validate operation codes and nonce values.
* Create detailed custom error codes.
* Write unit tests for valid and invalid authorization scenarios.

---

# Project Overview

The Week 3 contract is named:

```text
week-03-owner-lock
```

The contract is a configurable Lock Script that supports two authorization roles:

```text
Primary Owner
Recovery Owner
```

The primary owner represents the normal account or user allowed to spend the protected Cell.

The recovery owner represents a secondary authority that may be used in a recovery operation.

The contract reads both owners from the script arguments and compares them with an identifier supplied in the transaction witness.

---

# Repository Structure

The Week 3 contract follows the generated CKB Rust contract structure:

```text
contracts/
└── week-03-owner-lock/
    ├── Cargo.toml
    ├── Makefile
    └── src/
        ├── lib.rs
        └── main.rs
```

The repository also contains the journal entry:

```text
journal/
└── week-03.md
```

The files serve the following purposes:

* `main.rs` contains the contract entry point and validation logic.
* `lib.rs` exposes contract functions for native testing.
* `Cargo.toml` defines dependencies and features.
* `Makefile` provides build, test, formatting, and cleanup commands.
* `week-03.md` documents the implementation and learning progress.

---

# Lock Scripts in Nervos CKB

A Lock Script defines the conditions that must be satisfied before a Cell can be consumed as an input.

In an account-based blockchain, ownership is usually represented by an account balance associated with an address.

In CKB, assets and application state are stored inside Cells. Each Cell contains a Lock Script that determines who may spend it.

A Lock Script commonly validates:

* Digital signatures
* Multisignature conditions
* Time locks
* Administrative permissions
* Recovery mechanisms
* Custom authorization rules

The Week 3 contract implements a simplified authorization system using owner identifiers.

This is useful for learning how Lock Scripts work, although a production contract should verify cryptographic signatures rather than plain identifiers.

---

# Lock Script and Type Script Comparison

A CKB Cell can contain both a Lock Script and an optional Type Script.

The Lock Script answers:

```text
Who is allowed to consume this Cell?
```

The Type Script answers:

```text
What rules must this Cell follow?
```

In my Week 2 project, the Profile Cell Validator was a Type Script. It validated the profile data stored in the Cell.

In Week 3, the Multi-Role Owner Lock Script controls whether the Cell may be consumed.

Together, these scripts can protect and validate the same Cell:

```text
Lock Script
Controls ownership and authorization

Type Script
Controls the structure and lifecycle of the stored data
```

---

# Contract Configuration

The Lock Script configuration is stored inside the current script’s arguments.

The script arguments contain:

```text
Version
Primary owner length
Recovery owner length
Primary owner identifier
Recovery owner identifier
```

The binary structure is:

```text
┌─────────┬────────────────┬─────────────────┬───────────────┬────────────────┐
│ Version │ Primary Length │ Recovery Length │ Primary Owner │ Recovery Owner │
│ 1 byte  │ 1 byte         │ 1 byte          │ Variable      │ Variable       │
└─────────┴────────────────┴─────────────────┴───────────────┴────────────────┘
```

An example logical configuration is:

```text
Version: 1
Primary owner: jimleston_osoi
Recovery owner: recovery_admin
```

Using script arguments makes the contract reusable.

The same contract code can protect different Cells with different owners without recompiling the binary.

---

# Witness Authorization Format

The transaction witness contains the authorization information submitted by the user.

The witness lock field contains:

```text
Version
Action
Nonce
Identifier length
Identifier
```

The binary structure is:

```text
┌─────────┬────────┬────────────┬───────────────────┬──────────────────┐
│ Version │ Action │ Nonce      │ Identifier Length │ Identifier       │
│ 1 byte  │ 1 byte │ 8 bytes LE │ 1 byte            │ Variable         │
└─────────┴────────┴────────────┴───────────────────┴──────────────────┘
```

The contract currently supports two actions:

```text
0x01 = Primary owner authorization
0x02 = Recovery owner authorization
```

The action determines which configured identifier should be used during validation.

---

# Validation Flow

The contract follows this validation process:

```text
Transaction begins validation
          |
          v
Load the currently executing Lock Script
          |
          v
Read and parse script arguments
          |
          v
Validate contract version and owner lengths
          |
          v
Load the first GroupInput witness
          |
          v
Extract the witness lock field
          |
          v
Parse action, nonce, and identifier
          |
          v
Check the requested authorization role
          |
          v
Compare the witness identifier with the configured owner
          |
     ┌────┴────┐
     |         |
   Match     Mismatch
     |         |
     v         v
  Success     Error
```

This separation makes the contract easier to understand, maintain, and test.

---

# Supported Authorization Modes

## Primary Owner Authorization

The primary-owner action is used during normal Cell consumption.

The witness must contain:

```text
Action: 0x01
Identifier: configured primary owner
```

If the witness identifier matches the primary owner stored in the script arguments, the contract returns success.

If it does not match, the contract returns:

```text
OwnerAuthorizationFailed
```

## Recovery Owner Authorization

The recovery action provides a secondary authorization path.

The witness must contain:

```text
Action: 0x02
Identifier: configured recovery owner
```

If the witness contains the correct recovery identifier, the transaction is accepted.

If a primary owner attempts to use the recovery action, or the witness contains another identifier, the contract returns:

```text
RecoveryAuthorizationFailed
```

---

# Versioning

The contract uses a version byte in both the script arguments and witness payload.

The current supported version is:

```text
1
```

Versioning allows the data format to evolve in future versions while maintaining compatibility with existing Cells.

For example, a future contract version could add:

* Cryptographic public-key hashes
* Signature algorithms
* Expiration timestamps
* Multiple recovery owners
* Role permissions
* Threshold authorization

If the contract receives an unsupported version, it rejects the transaction.

---

# Nonce Validation

The witness contains an eight-byte unsigned integer called a nonce.

The nonce is encoded using little-endian byte order.

The contract currently requires the nonce to be greater than zero.

This introduces the concept of replay protection, where each authorization attempt should use a unique or increasing value.

However, the Lock Script cannot remember previously used nonce values on its own.

True replay protection would require a companion Type Script or state Cell that stores the current nonce and verifies that the output nonce has been incremented correctly.

Therefore, the current implementation validates the nonce structure but does not provide complete replay protection.

---

# Binary Parsing

A major part of the Week 3 project involved safely parsing binary data.

The contract performs checks before reading each field.

It validates:

* Minimum header length
* Declared identifier lengths
* Total payload length
* Maximum identifier size
* Empty values
* Supported versions
* Supported actions
* Allowed characters
* Duplicate owner roles

The contract also uses checked arithmetic when calculating expected payload lengths.

This prevents malformed data from causing invalid memory access or unexpected behavior.

---

# Identifier Validation

Both owner identifiers must follow a restricted format.

Allowed characters include:

```text
A-Z
a-z
0-9
_
-
.
```

Examples of valid identifiers:

```text
jimleston_osoi
recovery-admin
owner.account
developer2026
```

Examples of invalid identifiers:

```text
jimleston osoi
owner@email
admin/recovery
```

The maximum supported identifier length is:

```text
64 bytes
```

Restricting the input format makes the contract behavior more predictable and reduces malformed configurations.

---

# Error Handling

The contract uses custom error codes to identify the specific reason validation failed.

Examples include:

```text
EmptyScriptArgs
InvalidScriptArgsLength
UnsupportedScriptVersion
EmptyPrimaryOwner
EmptyRecoveryOwner
OwnerIdentifierTooLong
MissingWitnessLock
InvalidWitnessLength
UnsupportedWitnessVersion
UnsupportedAction
InvalidNonce
OwnerAuthorizationFailed
RecoveryAuthorizationFailed
```

Using specific error codes makes debugging easier because developers can identify which rule rejected the transaction.

The entry point converts each error into an `i8` exit code that can be returned by the CKB virtual machine.

---

# Main Contract Components

The contract is divided into several functions.

## `program_entry`

This is the main entry point executed by CKB.

It calls the validation function and returns either:

```text
0 for success
```

or a custom nonzero error code.

## `validate`

This function connects the contract to the CKB transaction environment.

It:

* Loads the current script.
* Reads the script arguments.
* Loads the group-input witness.
* Extracts the witness lock field.
* Parses the authorization payload.
* Runs the authorization check.

## `parse_script_args`

This function parses the version, primary owner, and recovery owner from the script arguments.

It validates the configuration before returning a structured `LockConfiguration`.

## `parse_authorization_witness`

This function parses the version, action, nonce, and identifier from the witness lock field.

It returns a structured `AuthorizationWitness`.

## `validate_authorization`

This function selects the expected owner based on the requested action.

It then compares the witness identifier with the configured owner.

## `validate_identifier`

This function validates owner length and supported characters.

---

# Data Structures

The contract uses two main Rust structures.

## Lock Configuration

```rust
pub struct LockConfiguration {
    pub version: u8,
    pub primary_owner: Vec<u8>,
    pub recovery_owner: Vec<u8>,
}
```

This structure represents the configuration stored in script arguments.

## Authorization Witness

```rust
pub struct AuthorizationWitness {
    pub version: u8,
    pub action: u8,
    pub nonce: u64,
    pub identifier: Vec<u8>,
}
```

This structure represents the authorization request stored in the witness.

Using structures separates data parsing from business logic and improves code readability.

---

# Testing Strategy

The contract includes native Rust unit tests.

The tests cover both valid and invalid scenarios.

## Script Argument Tests

The script configuration tests include:

* Valid configuration
* Empty script arguments
* Unsupported version
* Empty primary owner
* Empty recovery owner
* Duplicate primary and recovery owners
* Invalid payload length
* Oversized identifiers

## Witness Tests

The witness tests include:

* Valid primary-owner witness
* Valid recovery-owner witness
* Unsupported action
* Unsupported version
* Zero nonce
* Empty identifier
* Truncated witness
* Incorrect identifier length
* Invalid identifier characters

## Authorization Tests

The authorization tests include:

* Correct primary owner
* Incorrect primary owner
* Correct recovery owner
* Incorrect recovery owner
* Primary owner using the recovery action

These tests ensure that parsing and authorization logic behave correctly before the contract is executed in the CKB virtual machine.

---

# Challenges Encountered

## Understanding Witness Data

One challenge was understanding how transaction witnesses are related to input Cells.

The witness is not stored permanently inside the Cell. Instead, it is supplied by the transaction creator and can contain signatures or other authorization data.

The Lock Script reads this witness information when deciding whether the Cell can be consumed.

## Designing a Binary Format

Another challenge was creating a clear binary format for script arguments and witnesses.

The format needed to support variable-length identifiers while still allowing the contract to determine where each field starts and ends.

I addressed this by including explicit length fields in the payload.

## Validating Dynamic Data

Because the identifiers have variable lengths, the contract must check every offset and expected payload length before slicing the input data.

This helped me understand why careful bounds checking is important in smart contract development.

## Understanding Nonce Limitations

I initially considered the nonce field complete replay protection.

During implementation, I learned that a Lock Script cannot retain state between transactions.

A stateful Type Script or nonce Cell would be required to verify that a nonce has not already been used.

---

# Key Learnings

By completing Week 3, I learned that:

* Lock Scripts define who may consume a Cell.
* Type Scripts define application-specific rules.
* Script arguments provide reusable contract configuration.
* Witnesses carry authorization information.
* Group-input witnesses are used during Lock Script execution.
* Versioned formats make contracts easier to upgrade.
* Binary parsing requires strict length validation.
* Error codes improve debugging.
* A nonce alone does not prevent replay unless it is connected to on-chain state.
* Production authorization should use cryptographic proof rather than readable identifiers.

---

# Security Considerations

The Week 3 contract is intended for education and demonstration.

It should not be used to protect real CKB assets because it compares plain identifiers.

Anyone who knows the expected identifier could reproduce the witness payload.

A production-ready Lock Script should use stronger authorization methods, such as:

* Secp256k1 signature verification
* Public-key hash validation
* Multisignature authorization
* WebAuthn
* Hardware wallet signatures
* Time-based recovery rules
* Threshold signatures

The current project focuses on the validation structure that would later support these stronger mechanisms.

---

# Reflection

Week 3 significantly improved my understanding of CKB transaction authorization.

In Week 2, I focused on validating Cell data using a Type Script. During Week 3, I learned how Lock Scripts protect Cells and how transaction witnesses provide proof of authorization.

The multi-role design made the project more advanced than a simple owner comparison. It introduced versioning, binary serialization, different operation modes, nonce validation, structured parsing, and detailed error handling.

The project also demonstrated the importance of separating parsing, validation, and authorization logic.

This structure will make it easier to replace the readable identifiers with cryptographic signatures in future versions.

---

# Week 4 Goals

For Week 4, I plan to combine the concepts from Week 2 and Week 3 into a more complete CKB application.

The planned objectives include:

* Combine the Profile Type Script with the Owner Lock Script.
* Allow only the owner to update a profile Cell.
* Add a stateful nonce to the profile Cell.
* Require the output nonce to equal the input nonce plus one.
* Prevent unauthorized profile updates.
* Explore Molecule serialization.
* Write CKB integration tests using `ckb-testtool`.
* Build complete mock transactions.
* Measure contract cycles.
* Improve the recovery authorization model.

---

# Conclusion

Week 3 focused on building a configurable multi-role Lock Script for Nervos CKB.

The contract supports primary-owner and recovery-owner authorization, structured script arguments, structured witness data, versioning, action codes, nonce validation, custom errors, and extensive unit testing.

This project strengthened my understanding of ownership, transaction witnesses, script arguments, binary parsing, and authorization in the CKB Cell Model.

It also created a strong foundation for Week 4, where the Lock Script and Profile Type Script can be combined into a complete owner-controlled on-chain profile application.
