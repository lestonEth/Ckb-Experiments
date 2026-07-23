#![cfg_attr(not(feature = "library"), no_std)]
#![allow(special_module_name)]
#![allow(unused_attributes)]

extern crate alloc;

#[cfg(feature = "library")]
mod main;

#[cfg(feature = "library")]
pub use main::{
    parse_authorization_witness,
    parse_script_args,
    program_entry,
    validate,
    validate_authorization,
    validate_identifier,
    AuthorizationWitness,
    Error,
    LockConfiguration,
};