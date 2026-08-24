#![cfg_attr(not(feature = "library"), no_std)]
#![allow(special_module_name)]

extern crate alloc;

#[cfg(feature = "library")]
mod main;

#[cfg(feature = "library")]
pub use main::program_entry;

pub mod cell;
pub mod errors;
pub mod transaction;
pub mod validator;
pub mod witness;

#[cfg(not(test))]
pub mod ckb_transaction;

#[cfg(not(test))]
pub mod runtime;
