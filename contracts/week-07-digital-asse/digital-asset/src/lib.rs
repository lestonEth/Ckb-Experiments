#![cfg_attr(
    not(feature = "library"),
    no_std
)]

extern crate alloc;

pub mod asset;
pub mod errors;
pub mod validator;

#[cfg(feature = "library")]
mod main;

#[cfg(feature = "library")]
pub use main::program_entry;
