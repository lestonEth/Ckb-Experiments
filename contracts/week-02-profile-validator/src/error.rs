use ckb_std::error::SysError;

/// Error codes returned by the Week 2 Profile Validator.
///
/// CKB considers exit code 0 successful.
/// Every non-zero code represents a failed validation.
#[derive(Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum Error {
    // Standard syscall errors.
    IndexOutOfBound = 1,
    ItemMissing = 2,
    LengthNotEnough = 3,
    Encoding = 4,

    // Custom Week 2 errors.
    InvalidInputCount = 10,
    InvalidOutputCount = 11,
    ProfileDestructionNotAllowed = 12,
    InvalidProfilePrefix = 13,
    EmptyProfileName = 14,
    ProfileDataTooLong = 15,
    InvalidNameCharacter = 16,
}

impl From<SysError> for Error {
    fn from(error: SysError) -> Self {
        match error {
            SysError::IndexOutOfBound => Self::IndexOutOfBound,
            SysError::ItemMissing => Self::ItemMissing,
            SysError::LengthNotEnough(_) => Self::LengthNotEnough,
            SysError::Encoding => Self::Encoding,
            SysError::Unknown(code) => panic!("unexpected CKB syscall error: {}", code),
        }
    }
}