#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    EmptyInputs,
    EmptyOutputs,

    InvalidTransactionVersion,

    OutputsDataMismatch,

    CapacityOverflow,
    InsufficientCapacity,

    DuplicateInput,
    MissingInput,

    InvalidCellDep,
    DuplicateCellDep,

    InvalidHeaderDep,

    InvalidTransaction,

    InvalidCounterData,
    InvalidCounterTransition,
    CounterOverflow,

    InvalidWitness,
    EmptyWitness,

    InvalidSince,

    InputOutputDataMismatch,
}
