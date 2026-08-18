#![cfg_attr(not(any(feature = "library", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "library", test))]
extern crate alloc;

#[cfg(not(any(feature = "library", test)))]
use ckb_std::ckb_types::prelude::*;

#[cfg(not(any(feature = "library", test)))]
use ckb_std::high_level::load_script;

#[cfg(not(any(feature = "library", test)))]
use ckb_std::entry;

#[cfg(not(any(feature = "library", test)))]
ckb_std::default_alloc!(16384, 1258306, 64);

#[cfg(not(any(feature = "library", test)))]
entry!(program_entry);

#[cfg(not(any(feature = "library", test)))]
pub fn program_entry() -> i8 {
    ckb_std::debug!("Counter contract started");

    /*
     * Week 5 initial on-chain entry point.
     *
     * The full input/output Cell validation will be added
     * after the basic counter state-transition logic has
     * been verified through unit tests.
     *
     * Loading the current script also verifies that the
     * contract is executing inside the CKB-VM environment.
     */

    match load_script() {
        Ok(_) => {
            ckb_std::debug!("Counter script loaded successfully");
            0
        }

        Err(err) => {
            ckb_std::debug!(
                "Failed to load counter script: {:?}",
                err
            );

            -1
        }
    }
}