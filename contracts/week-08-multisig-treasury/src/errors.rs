#![no_std]

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    // Configuration errors
    InvalidConfigLength = 1,
    UnsupportedVersion = 2,
    InvalidThreshold = 3,
    InvalidSignerCount = 4,
    DuplicateSigner = 5,
    EmptySigner = 6,

    // Witness errors
    InvalidWitnessLength = 10,
    InvalidWitnessVersion = 11,
    InvalidSignatureCount = 12,
    DuplicateAuthorization = 13,
    UnknownSigner = 14,

    // Authorization errors
    InsufficientApprovals = 20,
    InvalidAuthorization = 21,

    // Transaction errors
    InvalidOutputCount = 30,
    UnauthorizedOutput = 31,
    InvalidStateTransition = 32,

    // General
    ScriptError = 40,
}

impl Error {
    pub fn code(self) -> i8 {
        self as i8
    }
}
