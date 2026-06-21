#![cfg_attr(not(feature = "library"), no_std)]
#![cfg_attr(not(feature = "library"), no_main)]

#[cfg(not(feature = "library"))]
ckb_std::entry!(program_entry);

#[cfg(not(feature = "library"))]
ckb_std::default_alloc!();


pub fn program_entry() -> i8 {
   ckb_std::debug!("Hello World");
0
}
