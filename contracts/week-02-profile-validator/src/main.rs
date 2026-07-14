#![cfg_attr(not(any(feature = "library", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "library", test))]
extern crate alloc;

use ckb_std::{
    ckb_constants::Source,
    error::SysError,
    high_level::load_cell_data,
};

#[cfg(not(any(feature = "library", test)))]
ckb_std::entry!(program_entry);

#[cfg(not(any(feature = "library", test)))]
ckb_std::default_alloc!(16384, 1258306, 64);

const PROFILE_PREFIX: &[u8] = b"CKB_PROFILE_V1|name=";
const MAX_PROFILE_DATA_LENGTH: usize = 128;

#[derive(Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing = 2,
    LengthNotEnough = 3,
    Encoding = 4,

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
            SysError::Unknown(code) => {
                ckb_std::debug!("Unknown syscall error: {}", code);
                Self::Encoding
            }
        }
    }
}

pub fn program_entry() -> i8 {
    match validate() {
        Ok(()) => {
            ckb_std::debug!("Week 2 profile validation passed");
            0
        }
        Err(error) => {
            ckb_std::debug!("Week 2 profile validation failed: {:?}", error);
            error as i8
        }
    }
}

pub fn validate() -> Result<(), Error> {
    let input_count = count_group_cells(Source::GroupInput)?;
    let output_count = count_group_cells(Source::GroupOutput)?;

    ckb_std::debug!(
        "Profile group input count: {}, output count: {}",
        input_count,
        output_count
    );

    if input_count > 1 {
        return Err(Error::InvalidInputCount);
    }

    if output_count == 0 {
        return Err(Error::ProfileDestructionNotAllowed);
    }

    if output_count > 1 {
        return Err(Error::InvalidOutputCount);
    }

    let output_data = load_cell_data(0, Source::GroupOutput)?;

    validate_profile_data(&output_data)?;

    match input_count {
        0 => {
            ckb_std::debug!("Creating a new profile Cell");
        }
        1 => {
            let input_data = load_cell_data(0, Source::GroupInput)?;

            ckb_std::debug!(
                "Updating profile Cell. Previous bytes: {}, new bytes: {}",
                input_data.len(),
                output_data.len()
            );
        }
        _ => {
            return Err(Error::InvalidInputCount);
        }
    }

    Ok(())
}

fn count_group_cells(source: Source) -> Result<usize, Error> {
    let mut index = 0;

    loop {
        match load_cell_data(index, source) {
            Ok(_) => {
                index += 1;
            }
            Err(SysError::IndexOutOfBound) => {
                break;
            }
            Err(error) => {
                return Err(Error::from(error));
            }
        }
    }

    Ok(index)
}

pub fn validate_profile_data(data: &[u8]) -> Result<(), Error> {
    if data.len() > MAX_PROFILE_DATA_LENGTH {
        return Err(Error::ProfileDataTooLong);
    }

    if !data.starts_with(PROFILE_PREFIX) {
        return Err(Error::InvalidProfilePrefix);
    }

    let name = &data[PROFILE_PREFIX.len()..];

    if name.is_empty() {
        return Err(Error::EmptyProfileName);
    }

    for character in name {
        if !is_allowed_name_character(*character) {
            return Err(Error::InvalidNameCharacter);
        }
    }

    Ok(())
}

fn is_allowed_name_character(character: u8) -> bool {
    matches!(
        character,
        b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'_'
            | b'-'
            | b' '
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_profile_data() {
        let result =
            validate_profile_data(b"CKB_PROFILE_V1|name=Jimleston_Osoi");

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn accepts_name_with_spaces() {
        let result =
            validate_profile_data(b"CKB_PROFILE_V1|name=Jimleston Osoi");

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn accepts_name_with_numbers() {
        let result =
            validate_profile_data(b"CKB_PROFILE_V1|name=Jimleston2026");

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn rejects_invalid_prefix() {
        let result =
            validate_profile_data(b"INVALID_PROFILE|name=Jimleston");

        assert_eq!(result, Err(Error::InvalidProfilePrefix));
    }

    #[test]
    fn rejects_empty_name() {
        let result = validate_profile_data(b"CKB_PROFILE_V1|name=");

        assert_eq!(result, Err(Error::EmptyProfileName));
    }

    #[test]
    fn rejects_invalid_character() {
        let result =
            validate_profile_data(b"CKB_PROFILE_V1|name=Jimleston@Osoi");

        assert_eq!(result, Err(Error::InvalidNameCharacter));
    }

    #[test]
    fn rejects_slash_character() {
        let result =
            validate_profile_data(b"CKB_PROFILE_V1|name=Jimleston/Osoi");

        assert_eq!(result, Err(Error::InvalidNameCharacter));
    }

    #[test]
    fn rejects_oversized_profile_data() {
        let mut data = alloc::vec::Vec::from(PROFILE_PREFIX);
        data.extend(alloc::vec![b'A'; 120]);

        assert!(data.len() > MAX_PROFILE_DATA_LENGTH);

        let result = validate_profile_data(&data);

        assert_eq!(result, Err(Error::ProfileDataTooLong));
    }
}