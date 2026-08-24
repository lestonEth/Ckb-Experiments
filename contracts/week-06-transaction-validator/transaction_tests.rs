#[cfg(test)]
mod tests {
    use crate::{
        errors::ValidationError,
        transaction::{
            CellDep,
            CellInput,
            CellOutput,
            DepType,
            OutPoint,
            Transaction,
        },
        validator::{
            detect_duplicate_inputs,
            validate_transaction,
            validate_version,
        },
    };

    fn hash(byte: u8) -> [u8; 32] {
        [byte; 32]
    }

    fn valid_out_point() -> OutPoint {
        OutPoint::new(
            hash(1),
            0,
        )
    }

    fn valid_input() -> CellInput {
        CellInput::new(
            valid_out_point(),
            0,
        )
    }

    fn valid_transaction() -> Transaction {
        Transaction::new(
            0,
            vec![
                CellDep::new(
                    OutPoint::new(
                        hash(2),
                        0,
                    ),
                    DepType::Code,
                )
            ],
            vec![],
            vec![
                valid_input()
            ],
            vec![
                CellOutput::new(
                    900,
                    hash(3),
                    None,
                )
            ],
            vec![
                vec![]
            ],
            vec![
                vec![]
            ],
        )
    }

    #[test]
    fn accepts_valid_transaction() {
        let tx = valid_transaction();

        assert_eq!(
            validate_transaction(
                &tx,
                1000
            ),
            Ok(())
        );
    }

    #[test]
    fn rejects_invalid_version() {
        let mut tx =
            valid_transaction();

        tx.version = 1;

        assert_eq!(
            validate_version(&tx),
            Err(
                ValidationError::InvalidTransactionVersion
            )
        );
    }

    #[test]
    fn rejects_duplicate_inputs() {
        let mut tx =
            valid_transaction();

        tx.inputs.push(
            valid_input()
        );

        assert_eq!(
            detect_duplicate_inputs(
                &tx.inputs
            ),
            Err(
                ValidationError::DuplicateInput
            )
        );
    }

    #[test]
    fn rejects_missing_input() {
        let mut tx =
            valid_transaction();

        tx.inputs[0] =
            CellInput::new(
                OutPoint::new(
                    [0u8; 32],
                    0,
                ),
                0,
            );

        assert_eq!(
            validate_transaction(
                &tx,
                1000
            ),
            Err(
                ValidationError::MissingInput
            )
        );
    }

    #[test]
    fn rejects_output_data_mismatch() {
        let mut tx =
            valid_transaction();

        tx.outputs_data.clear();

        assert_eq!(
            validate_transaction(
                &tx,
                1000
            ),
            Err(
                ValidationError::OutputsDataMismatch
            )
        );
    }

    #[test]
    fn rejects_capacity_creation() {
        let mut tx =
            valid_transaction();

        tx.outputs[0].capacity = 2000;

        assert_eq!(
            validate_transaction(
                &tx,
                1000
            ),
            Err(
                ValidationError::InsufficientCapacity
            )
        );
    }

    #[test]
    fn accepts_multiple_unique_inputs() {
        let mut tx =
            valid_transaction();

        tx.inputs.push(
            CellInput::new(
                OutPoint::new(
                    hash(10),
                    1,
                ),
                0,
            )
        );

        assert_eq!(
            detect_duplicate_inputs(
                &tx.inputs
            ),
            Ok(())
        );
    }

    #[test]
    fn rejects_duplicate_cell_dep() {
        let mut tx =
            valid_transaction();

        tx.cell_deps.push(
            CellDep::new(
                OutPoint::new(
                    hash(2),
                    0,
                ),
                DepType::Code,
            )
        );

        assert_eq!(
            crate::validator::validate_cell_deps(
                &tx.cell_deps
            ),
            Err(
                ValidationError::DuplicateCellDep
            )
        );
    }
}
