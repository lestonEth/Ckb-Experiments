# Week 1 Cell Validator Contract

This is my first concrete CKB contract for Week 1.

The contract validates that the first output cell contains a specific message:

```text
CKB_WEEK_1_COMPLETED
```

## What this contract proves

This contract shows that I can:

* Create a CKB Rust script
* Read output cell data from a transaction
* Compare cell data against an expected value
* Return success or failure using CKB script exit codes

## Validation Logic

The script reads data from the first output cell:

```rust
load_cell_data(0, Source::Output)
```

If the output data matches the expected message, the contract returns `0`.

If the data is missing or incorrect, the contract returns a non-zero error code.

## Error Codes

| Code | Meaning                                                |
| ---- | ------------------------------------------------------ |
| 0    | Validation passed                                      |
| 1    | Output data was not found                              |
| 2    | Output data does not match the expected Week 1 message |

## Learning Outcome

This helped me move from a basic “hello world” contract to a real validation script that checks transaction output data.
