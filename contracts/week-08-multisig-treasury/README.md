# Week 8 — Multisignature Treasury

## Overview

This project is the final project in my Nervos CKB learning journey.

The contract implements a simplified 2-of-3 multisignature authorization model.

Three authorized signers control a treasury:

- Alice
- Bob
- Charlie

At least two valid authorizations are required before an operation can be approved.

---

## Learning Objectives

This project focuses on:

- CKB Lock Scripts
- Script arguments
- Witness data
- Transaction authorization
- Multisignature authorization
- State validation
- Fraudulent transaction detection
- Custom error handling
- CKB script testing

---

## Authorization Model

The treasury uses a 2-of-3 threshold.

```text
Alice
Bob
Charlie

Required approvals: 2
