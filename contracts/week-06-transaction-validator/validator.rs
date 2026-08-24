use crate::{
    errors::ValidationError,
    transaction::{
        CellDep,
        CellInput,
        OutPoint,
        Transaction,
    },
};

pub fn validate_version(
    transaction: &Transaction,
) -> Result<(), ValidationError> {
    if transaction.version != 0 {
        return Err(
            ValidationError::InvalidTransactionVersion
        );
    }

    Ok(())
}

pub fn validate_capacity(
    input_capacity: u64,
    transaction: &Transaction,
) -> Result<(), ValidationError> {
    let output_capacity = transaction
        .outputs
        .iter()
        .try_fold(0u64, |total, output| {
            total.checked_add(output.capacity)
        })
        .ok_or(
            ValidationError::CapacityOverflow
        )?;

    if output_capacity > input_capacity {
        return Err(
            ValidationError::InsufficientCapacity
        );
    }

    Ok(())
}

pub fn detect_duplicate_inputs(
    inputs: &[CellInput],
) -> Result<(), ValidationError> {
    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            if inputs[i].previous_output
                == inputs[j].previous_output
            {
                return Err(
                    ValidationError::DuplicateInput
                );
            }
        }
    }

    Ok(())
}

pub fn validate_inputs_exist(
    inputs: &[CellInput],
) -> Result<(), ValidationError> {
    for input in inputs {
        if input.previous_output.is_zero() {
            return Err(
                ValidationError::MissingInput
            );
        }
    }

    Ok(())
}

pub fn validate_cell_deps(
    cell_deps: &[CellDep],
) -> Result<(), ValidationError> {
    for i in 0..cell_deps.len() {
        if cell_deps[i].out_point.is_zero() {
            return Err(
                ValidationError::InvalidCellDep
            );
        }

        for j in (i + 1)..cell_deps.len() {
            if cell_deps[i].out_point
                == cell_deps[j].out_point
            {
                return Err(
                    ValidationError::DuplicateCellDep
                );
            }
        }
    }

    Ok(())
}

pub fn validate_header_deps(
    header_deps: &[OutPoint],
) -> Result<(), ValidationError> {
    for header in header_deps {
        if header.tx_hash == [0u8; 32] {
            return Err(
                ValidationError::InvalidHeaderDep
            );
        }
    }

    Ok(())
}

pub fn validate_outputs_data(
    transaction: &Transaction,
) -> Result<(), ValidationError> {
    if transaction.outputs.len()
        != transaction.outputs_data.len()
    {
        return Err(
            ValidationError::OutputsDataMismatch
        );
    }

    Ok(())
}

pub fn validate_counter_transition(
    input_counter: u64,
    output_counter: u64,
) -> Result<(), ValidationError> {
    let expected = input_counter
        .checked_add(1)
        .ok_or(
            ValidationError::CounterOverflow
        )?;

    if output_counter != expected {
        return Err(
            ValidationError::InvalidCounterTransition
        );
    }

    Ok(())
}

pub fn validate_transaction(
    transaction: &Transaction,
    input_capacity: u64,
) -> Result<(), ValidationError> {
    transaction.validate_structure()?;

    validate_version(transaction)?;

    detect_duplicate_inputs(
        &transaction.inputs
    )?;

    validate_inputs_exist(
        &transaction.inputs
    )?;

    validate_cell_deps(
        &transaction.cell_deps
    )?;

    validate_outputs_data(
        transaction
    )?;

    validate_capacity(
        input_capacity,
        transaction
    )?;

    Ok(())
}
