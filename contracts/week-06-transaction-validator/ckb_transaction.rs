use crate::transaction::{
    CellInput,
    CellOutput,
    Hash,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CkbTransaction {
    pub version: u32,

    pub cell_dep_count: usize,
    pub header_dep_count: usize,

    pub inputs: Vec<CellInput>,

    pub outputs: Vec<CellOutput>,
    pub outputs_data: Vec<Vec<u8>>,

    pub witnesses: Vec<Vec<u8>>,
}

impl CkbTransaction {
    pub fn new(
        version: u32,
        cell_dep_count: usize,
        header_dep_count: usize,
        inputs: Vec<CellInput>,
        outputs: Vec<CellOutput>,
        outputs_data: Vec<Vec<u8>>,
        witnesses: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            version,
            cell_dep_count,
            header_dep_count,
            inputs,
            outputs,
            outputs_data,
            witnesses,
        }
    }

    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }

    pub fn witness_count(&self) -> usize {
        self.witnesses.len()
    }

    pub fn has_outputs_data_for_every_output(
        &self,
    ) -> bool {
        self.outputs.len()
            == self.outputs_data.len()
    }

    pub fn total_output_capacity(
        &self,
    ) -> Option<u64> {
        self.outputs
            .iter()
            .try_fold(0u64, |total, output| {
                total.checked_add(output.capacity)
            })
    }

    pub fn output_data(
        &self,
        index: usize,
    ) -> Option<&[u8]> {
        self.outputs_data
            .get(index)
            .map(|data| data.as_slice())
    }

    pub fn witness(
        &self,
        index: usize,
    ) -> Option<&[u8]> {
        self.witnesses
            .get(index)
            .map(|data| data.as_slice())
    }
}
