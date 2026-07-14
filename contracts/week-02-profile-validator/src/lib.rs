#![no_std]

mod error;

use ckb_std::{
    ckb_constants::Source,
    debug,
    error::SysError,
    high_level::load_cell_data,
};

pub use error::Error;

/// Every profile Cell must start with this versioned prefix.
const PROFILE_PREFIX: &[u8] = b"CKB_PROFILE_V1|name=";

/// Maximum size allowed for the full profile Cell data.
const MAX_PROFILE_DATA_LENGTH: usize = 128;

/// Main validation function.
///
/// Rules:
/// 1. At most one matching profile Cell may appear in group inputs.
/// 2. Exactly one matching profile Cell must appear in group outputs.
/// 3. Therefore, profile creation and update are allowed.
/// 4. Profile destruction is not allowed.
/// 5. Output data must follow the expected profile format.
pub fn validate() -> Result<(), Error> {
    let input_count = count_group_cells(Source::GroupInput)?;
    let output_count = count_group_cells(Source::GroupOutput)?;

    debug!(
        "Week 2 profile validator: inputs={}, outputs={}",
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

    if input_count == 0 {
        debug!("Creating a new CKB profile Cell");
    } else {
        debug!("Updating an existing CKB profile Cell");

        let input_data = load_cell_data(0, Source::GroupInput)?;

        debug!(
            "Profile update: previous length={}, new length={}",
            input_data.len(),
            output_data.len()
        );
    }

    Ok(())
}

/// Counts Cells in a script group.
///
/// `Source::GroupInput` means inputs using the currently executing script.
/// `Source::GroupOutput` means outputs using the same script.
fn count_group_cells(source: Source) -> Result<usize, Error> {
    let mut count = 0;

    loop {
        match load_cell_data(count, source) {
            Ok(_) => {
                count += 1;
            }
            Err(SysError::IndexOutOfBound) => {
                break;
            }
            Err(error) => {
                return Err(Error::from(error));
            }
        }
    }

    Ok(count)
}

/// Validates the bytes stored in the output profile Cell.
fn validate_profile_data(data: &[u8]) -> Result<(), Error> {
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

/// Keeps the Week 2 parser intentionally small and deterministic.
///
/// Allowed:
/// - A-Z
/// - a-z
/// - 0-9
/// - underscore
/// - hyphen
/// - regular space
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
mod unit_tests {
    use super::*;

    #[test]
    fn accepts_a_valid_profile() {
        let result = validate_profile_data(
            b"CKB_PROFILE_V1|name=Jimleston_Osoi",
        );

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn rejects_an_empty_name() {
        let result = validate_profile_data(
            b"CKB_PROFILE_V1|name=",
        );

        assert_eq!(result, Err(Error::EmptyProfileName));
    }

    #[test]
    fn rejects_invalid_characters() {
        let result = validate_profile_data(
            b"CKB_PROFILE_V1|name=Jimleston@Osoi",
        );

        assert_eq!(result, Err(Error::InvalidNameCharacter));
    }

    #[test]
    fn rejects_an_invalid_prefix() {
        let result = validate_profile_data(
            b"WRONG_PROFILE|name=Jimleston",
        );

        assert_eq!(result, Err(Error::InvalidProfilePrefix));
    }
}