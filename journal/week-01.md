# Week 1: Building My First Concrete CKB Validator

## Overview

This week, I moved beyond a simple hello-world contract and built a small CKB validation script.

The contract validates that the first output cell in a transaction contains the message:

```text
CKB_WEEK_1_COMPLETED
```

## What I Built

I created a contract called:

```text
week-01-cell-validator
```

The contract reads data from the first output cell and checks whether it matches the expected Week 1 completion message.

## Main Concept Practiced

The main concept I practiced this week was reading cell data from a transaction.

In CKB, smart contracts do not simply update account balances like Ethereum. Instead, they validate cells. A transaction consumes old cells and creates new output cells.

This contract helped me understand that scripts can inspect transaction data and decide whether the transaction is valid.

## What I Learned

* CKB contracts are validation scripts
* Output cells can contain data
* Scripts can read output cell data
* Returning `0` means validation passed
* Returning a non-zero value means validation failed
* The Cell Model is central to how CKB works

## Contract Logic

The contract checks the first output cell.

If the output data is:

```text
CKB_WEEK_1_COMPLETED
```

the script passes.

If the output data is missing or different, the script fails.

## Reflection

This was a good first concrete step in learning CKB development. Instead of only compiling a basic contract, I now have a script that performs actual validation.

The next step is to learn how to pass script arguments and use them instead of hardcoding the expected message.
