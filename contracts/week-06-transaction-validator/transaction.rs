use crate::errors::ValidationError;

pub type Hash = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutPoint {
    pub tx_hash: Hash,
    pub index: u32,
}

impl OutPoint {
    pub fn new(tx_hash: Hash, index: u32) -> Self {
        Self { tx_hash, index }
    }

    pub fn is_zero(&self) -> bool {
        self.tx_hash == [0u8; 32]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellInput {
    pub previous_output: OutPoint,
    pub since: u64,
}

impl CellInput {
    pub fn new(
        previous_output: OutPoint,
        since: u64,
    ) -> Self {
        Self {
            previous_output,
            since,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepType {
    Code,
    DepGroup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellDep {
    pub out_point: OutPoint,
    pub dep_type: DepType,
}

impl CellDep {
    pub fn new(
        out_point: OutPoint,
        dep_type: DepType,
    ) -> Self {
        Self {
            out_point,
            dep_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellOutput {
    pub capacity: u64,
    pub lock_hash: Hash,
    pub type_hash: Option<Hash>,
}

impl CellOutput {
    pub fn new(
        capacity: u64,
        lock_hash: Hash,
        type_hash: Option<Hash>,
    ) -> Self {
        Self {
            capacity,
            lock_hash,
            type_hash,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub version: u32,
    pub cell_deps: Vec<CellDep>,
    pub header_deps: Vec<Hash>,
    pub inputs: Vec<CellInput>,
    pub outputs: Vec<CellOutput>,
    pub outputs_data: Vec<Vec<u8>>,
    pub witnesses: Vec<Vec<u8>>,
}

impl Transaction {
    pub fn new(
        version: u32,
        cell_deps: Vec<CellDep>,
        header_deps: Vec<Hash>,
        inputs: Vec<CellInput>,
        outputs: Vec<CellOutput>,
        outputs_data: Vec<Vec<u8>>,
        witnesses: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            version,
            cell_deps,
            header_deps,
            inputs,
            outputs,
            outputs_data,
            witnesses,
        }
    }

    pub fn validate_structure(
        &self,
    ) -> Result<(), ValidationError> {
        if self.inputs.is_empty() {
            return Err(ValidationError::EmptyInputs);
        }

        if self.outputs.is_empty() {
            return Err(ValidationError::EmptyOutputs);
        }

        if self.outputs.len() != self.outputs_data.len() {
            return Err(
                ValidationError::OutputsDataMismatch
            );
        }

        Ok(())
    }
}
