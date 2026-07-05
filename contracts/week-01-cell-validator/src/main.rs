#![no_std]
#![no_main]

use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::*,
    debug,
    default_alloc,
    error::SysError,
    high_level::load_cell_data,
};

ckb_std::entry!(program_entry);
default_alloc!();

const EXPECTED_MESSAGE: &[u8] = b"CKB_WEEK_1_COMPLETED";

#[repr(i8)]
enum Error {
    OutputDataNotFound = 1,
    InvalidWeekOneMessage = 2,
}

fn program_entry() -> i8 {
    match validate_week_one_output() {
        Ok(_) => {
            debug!("Week 1 validation passed");
            0
        }
        Err(error) => error as i8,
    }
}

fn validate_week_one_output() -> Result<(), Error> {
    let output_data = load_cell_data(0, Source::Output)
        .map_err(|err| match err {
            SysError::IndexOutOfBound => Error::OutputDataNotFound,
            _ => Error::InvalidWeekOneMessage,
        })?;

    if output_data.as_slice() != EXPECTED_MESSAGE {
        return Err(Error::InvalidWeekOneMessage);
    }

    Ok(())
}