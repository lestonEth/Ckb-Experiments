#![cfg(not(test))]

use alloc::vec::Vec;

use ckb_std::{
    ckb_constants::Source,
    syscalls::{
        load_cell_by_field,
        load_cell_data,
        load_input_by_field,
        load_transaction,
        load_tx_hash,
        load_witness,
    },
};

use crate::errors::ValidationError;

const MAX_BUFFER_SIZE: usize = 4096;

pub struct RuntimeTransaction {
    pub tx_hash: [u8; 32],

    pub input_capacities: Vec<u64>,
    pub output_capacities: Vec<u64>,

    pub input_data: Vec<Vec<u8>>,
    pub output_data: Vec<Vec<u8>>,

    pub witnesses: Vec<Vec<u8>>,
}

impl RuntimeTransaction {
    pub fn load() -> Result<Self, ValidationError> {
        let tx_hash = load_tx_hash()
            .map_err(|_| {
                ValidationError::InvalidTransaction
            })?;

        let input_capacities =
            load_input_capacities()?;

        let output_capacities =
            load_output_capacities()?;

        let output_data =
            load_all_output_data()?;

        let witnesses =
            load_all_witnesses()?;

        let input_data =
            load_all_input_data()?;

        Ok(Self {
            tx_hash,
            input_capacities,
            output_capacities,
            input_data,
            output_data,
            witnesses,
        })
    }

    pub fn input_count(&self) -> usize {
        self.input_capacities.len()
    }

    pub fn output_count(&self) -> usize {
        self.output_capacities.len()
    }

    pub fn total_input_capacity(
        &self,
    ) -> Result<u64, ValidationError> {
        self.input_capacities
            .iter()
            .try_fold(0u64, |total, capacity| {
                total.checked_add(*capacity)
            })
            .ok_or(
                ValidationError::CapacityOverflow
            )
    }

    pub fn total_output_capacity(
        &self,
    ) -> Result<u64, ValidationError> {
        self.output_capacities
            .iter()
            .try_fold(0u64, |total, capacity| {
                total.checked_add(*capacity)
            })
            .ok_or(
                ValidationError::CapacityOverflow
            )
    }
}

fn load_input_capacities(
) -> Result<Vec<u64>, ValidationError> {
    let mut result = Vec::new();

    for index in 0.. {
        let mut buffer = [0u8; 8];
        let mut len = 8u64;

        let ret = load_cell_by_field(
            &mut buffer,
            &mut len,
            0,
            index,
            Source::Input,
            ckb_std::ckb_constants::CellField::Capacity,
        );

        if ret.is_err() {
            break;
        }

        if len != 8 {
            return Err(
                ValidationError::InvalidTransaction
            );
        }

        result.push(
            u64::from_le_bytes(buffer)
        );
    }

    if result.is_empty() {
        return Err(
            ValidationError::EmptyInputs
        );
    }

    Ok(result)
}

fn load_output_capacities(
) -> Result<Vec<u64>, ValidationError> {
    let mut result = Vec::new();

    for index in 0.. {
        let mut buffer = [0u8; 8];
        let mut len = 8u64;

        let ret = load_cell_by_field(
            &mut buffer,
            &mut len,
            0,
            index,
            Source::Output,
            ckb_std::ckb_constants::CellField::Capacity,
        );

        if ret.is_err() {
            break;
        }

        if len != 8 {
            return Err(
                ValidationError::InvalidTransaction
            );
        }

        result.push(
            u64::from_le_bytes(buffer)
        );
    }

    if result.is_empty() {
        return Err(
            ValidationError::EmptyOutputs
        );
    }

    Ok(result)
}

fn load_all_output_data(
) -> Result<Vec<Vec<u8>>, ValidationError> {
    let mut result = Vec::new();

    for index in 0.. {
        let mut buffer =
            [0u8; MAX_BUFFER_SIZE];

        let mut len =
            MAX_BUFFER_SIZE as u64;

        let ret = load_cell_data(
            &mut buffer,
            &mut len,
            0,
            index,
            Source::Output,
        );

        if ret.is_err() {
            break;
        }

        result.push(
            buffer[..len as usize].to_vec()
        );
    }

    Ok(result)
}

fn load_all_input_data(
) -> Result<Vec<Vec<u8>>, ValidationError> {
    let mut result = Vec::new();

    for index in 0.. {
        let mut buffer =
            [0u8; MAX_BUFFER_SIZE];

        let mut len =
            MAX_BUFFER_SIZE as u64;

        let ret = load_cell_data(
            &mut buffer,
            &mut len,
            0,
            index,
            Source::Input,
        );

        if ret.is_err() {
            break;
        }

        result.push(
            buffer[..len as usize].to_vec()
        );
    }

    Ok(result)
}

fn load_all_witnesses(
) -> Result<Vec<Vec<u8>>, ValidationError> {
    let mut result = Vec::new();

    for index in 0.. {
        let mut buffer =
            [0u8; MAX_BUFFER_SIZE];

        let mut len =
            MAX_BUFFER_SIZE as u64;

        let ret = load_witness(
            &mut buffer,
            &mut len,
            0,
            index,
            Source::Input,
        );

        if ret.is_err() {
            break;
        }

        result.push(
            buffer[..len as usize].to_vec()
        );
    }

    Ok(result)
}
