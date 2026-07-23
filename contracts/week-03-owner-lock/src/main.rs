#![cfg_attr(not(any(feature = "library", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "library", test))]
extern crate alloc;

use alloc::vec::Vec;

use ckb_std::{
    ckb_constants::Source,
    error::SysError,
    high_level::{load_script, load_witness_args},
};

#[cfg(not(any(feature = "library", test)))]
ckb_std::entry!(program_entry);

#[cfg(not(any(feature = "library", test)))]
ckb_std::default_alloc!(16384, 1258306, 64);

const CONTRACT_VERSION: u8 = 1;

const ACTION_OWNER: u8 = 1;
const ACTION_RECOVERY: u8 = 2;

const MAX_IDENTIFIER_LENGTH: usize = 64;

const SCRIPT_ARGS_HEADER_LENGTH: usize = 3;
const WITNESS_HEADER_LENGTH: usize = 11;

#[derive(Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing = 2,
    LengthNotEnough = 3,
    Encoding = 4,

    EmptyScriptArgs = 10,
    InvalidScriptArgsLength = 11,
    UnsupportedScriptVersion = 12,
    EmptyPrimaryOwner = 13,
    EmptyRecoveryOwner = 14,
    OwnerIdentifierTooLong = 15,
    RecoveryIdentifierTooLong = 16,
    InvalidOwnerCharacter = 17,

    MissingWitnessLock = 20,
    EmptyWitnessLock = 21,
    InvalidWitnessLength = 22,
    UnsupportedWitnessVersion = 23,
    UnsupportedAction = 24,
    EmptyWitnessIdentifier = 25,
    WitnessIdentifierTooLong = 26,
    IdentifierLengthMismatch = 27,

    OwnerAuthorizationFailed = 30,
    RecoveryAuthorizationFailed = 31,
    InvalidNonce = 32,
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

#[derive(Debug, PartialEq, Eq)]
pub struct LockConfiguration {
    pub version: u8,
    pub primary_owner: Vec<u8>,
    pub recovery_owner: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AuthorizationWitness {
    pub version: u8,
    pub action: u8,
    pub nonce: u64,
    pub identifier: Vec<u8>,
}

pub fn program_entry() -> i8 {
    match validate() {
        Ok(()) => {
            ckb_std::debug!("Week 3 authorization succeeded");
            0
        }
        Err(error) => {
            ckb_std::debug!("Week 3 authorization failed: {:?}", error);
            error as i8
        }
    }
}

pub fn validate() -> Result<(), Error> {
    let script = load_script()?;
    let raw_script_args = script.args().raw_data();

    let configuration = parse_script_args(raw_script_args.as_ref())?;

    ckb_std::debug!(
        "Loaded lock configuration. Version: {}, primary owner length: {}, recovery owner length: {}",
        configuration.version,
        configuration.primary_owner.len(),
        configuration.recovery_owner.len()
    );

    let witness_args = load_witness_args(0, Source::GroupInput)?;

    let witness_lock = witness_args
        .lock()
        .to_opt()
        .ok_or(Error::MissingWitnessLock)?;

    let raw_witness = witness_lock.raw_data();

    if raw_witness.is_empty() {
        return Err(Error::EmptyWitnessLock);
    }

    let authorization = parse_authorization_witness(raw_witness.as_ref())?;

    validate_authorization(&configuration, &authorization)
}

pub fn parse_script_args(
    data: &[u8],
) -> Result<LockConfiguration, Error> {
    if data.is_empty() {
        return Err(Error::EmptyScriptArgs);
    }

    if data.len() < SCRIPT_ARGS_HEADER_LENGTH {
        return Err(Error::InvalidScriptArgsLength);
    }

    let version = data[0];

    if version != CONTRACT_VERSION {
        return Err(Error::UnsupportedScriptVersion);
    }

    let primary_owner_length = data[1] as usize;
    let recovery_owner_length = data[2] as usize;

    if primary_owner_length == 0 {
        return Err(Error::EmptyPrimaryOwner);
    }

    if recovery_owner_length == 0 {
        return Err(Error::EmptyRecoveryOwner);
    }

    if primary_owner_length > MAX_IDENTIFIER_LENGTH {
        return Err(Error::OwnerIdentifierTooLong);
    }

    if recovery_owner_length > MAX_IDENTIFIER_LENGTH {
        return Err(Error::RecoveryIdentifierTooLong);
    }

    let expected_length = SCRIPT_ARGS_HEADER_LENGTH
        .checked_add(primary_owner_length)
        .and_then(|length| length.checked_add(recovery_owner_length))
        .ok_or(Error::InvalidScriptArgsLength)?;

    if data.len() != expected_length {
        return Err(Error::InvalidScriptArgsLength);
    }

    let primary_owner_start = SCRIPT_ARGS_HEADER_LENGTH;
    let primary_owner_end = primary_owner_start + primary_owner_length;

    let recovery_owner_start = primary_owner_end;
    let recovery_owner_end = recovery_owner_start + recovery_owner_length;

    let primary_owner =
        data[primary_owner_start..primary_owner_end].to_vec();

    let recovery_owner =
        data[recovery_owner_start..recovery_owner_end].to_vec();

    validate_identifier(&primary_owner)?;
    validate_identifier(&recovery_owner)?;

    if primary_owner == recovery_owner {
        return Err(Error::InvalidScriptArgsLength);
    }

    Ok(LockConfiguration {
        version,
        primary_owner,
        recovery_owner,
    })
}

pub fn parse_authorization_witness(
    data: &[u8],
) -> Result<AuthorizationWitness, Error> {
    if data.is_empty() {
        return Err(Error::EmptyWitnessLock);
    }

    if data.len() < WITNESS_HEADER_LENGTH {
        return Err(Error::InvalidWitnessLength);
    }

    let version = data[0];

    if version != CONTRACT_VERSION {
        return Err(Error::UnsupportedWitnessVersion);
    }

    let action = data[1];

    if action != ACTION_OWNER && action != ACTION_RECOVERY {
        return Err(Error::UnsupportedAction);
    }

    let nonce_bytes: [u8; 8] = data[2..10]
        .try_into()
        .map_err(|_| Error::InvalidWitnessLength)?;

    let nonce = u64::from_le_bytes(nonce_bytes);

    if nonce == 0 {
        return Err(Error::InvalidNonce);
    }

    let identifier_length = data[10] as usize;

    if identifier_length == 0 {
        return Err(Error::EmptyWitnessIdentifier);
    }

    if identifier_length > MAX_IDENTIFIER_LENGTH {
        return Err(Error::WitnessIdentifierTooLong);
    }

    let expected_length = WITNESS_HEADER_LENGTH
        .checked_add(identifier_length)
        .ok_or(Error::InvalidWitnessLength)?;

    if data.len() != expected_length {
        return Err(Error::IdentifierLengthMismatch);
    }

    let identifier = data[WITNESS_HEADER_LENGTH..].to_vec();

    validate_identifier(&identifier)?;

    Ok(AuthorizationWitness {
        version,
        action,
        nonce,
        identifier,
    })
}

pub fn validate_authorization(
    configuration: &LockConfiguration,
    authorization: &AuthorizationWitness,
) -> Result<(), Error> {
    match authorization.action {
        ACTION_OWNER => {
            if authorization.identifier != configuration.primary_owner {
                return Err(Error::OwnerAuthorizationFailed);
            }

            ckb_std::debug!(
                "Primary owner authorization accepted. Nonce: {}",
                authorization.nonce
            );

            Ok(())
        }

        ACTION_RECOVERY => {
            if authorization.identifier != configuration.recovery_owner {
                return Err(Error::RecoveryAuthorizationFailed);
            }

            ckb_std::debug!(
                "Recovery authorization accepted. Nonce: {}",
                authorization.nonce
            );

            Ok(())
        }

        _ => Err(Error::UnsupportedAction),
    }
}

pub fn validate_identifier(
    identifier: &[u8],
) -> Result<(), Error> {
    if identifier.is_empty() {
        return Err(Error::EmptyWitnessIdentifier);
    }

    if identifier.len() > MAX_IDENTIFIER_LENGTH {
        return Err(Error::WitnessIdentifierTooLong);
    }

    for character in identifier {
        if !is_allowed_identifier_character(*character) {
            return Err(Error::InvalidOwnerCharacter);
        }
    }

    Ok(())
}

fn is_allowed_identifier_character(character: u8) -> bool {
    matches!(
        character,
        b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'_'
            | b'-'
            | b'.'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_script_args(
        primary_owner: &[u8],
        recovery_owner: &[u8],
    ) -> Vec<u8> {
        let mut data = Vec::new();

        data.push(CONTRACT_VERSION);
        data.push(primary_owner.len() as u8);
        data.push(recovery_owner.len() as u8);
        data.extend_from_slice(primary_owner);
        data.extend_from_slice(recovery_owner);

        data
    }

    fn build_witness(
        action: u8,
        nonce: u64,
        identifier: &[u8],
    ) -> Vec<u8> {
        let mut data = Vec::new();

        data.push(CONTRACT_VERSION);
        data.push(action);
        data.extend_from_slice(&nonce.to_le_bytes());
        data.push(identifier.len() as u8);
        data.extend_from_slice(identifier);

        data
    }

    #[test]
    fn parses_valid_script_args() {
        let data =
            build_script_args(b"jimleston_osoi", b"recovery_admin");

        let result = parse_script_args(&data).unwrap();

        assert_eq!(result.version, CONTRACT_VERSION);
        assert_eq!(result.primary_owner, b"jimleston_osoi");
        assert_eq!(result.recovery_owner, b"recovery_admin");
    }

    #[test]
    fn rejects_empty_script_args() {
        let result = parse_script_args(b"");

        assert_eq!(result, Err(Error::EmptyScriptArgs));
    }

    #[test]
    fn rejects_unsupported_script_version() {
        let mut data =
            build_script_args(b"jimleston_osoi", b"recovery_admin");

        data[0] = 2;

        let result = parse_script_args(&data);

        assert_eq!(
            result,
            Err(Error::UnsupportedScriptVersion)
        );
    }

    #[test]
    fn rejects_empty_primary_owner() {
        let data = build_script_args(b"", b"recovery_admin");

        let result = parse_script_args(&data);

        assert_eq!(result, Err(Error::EmptyPrimaryOwner));
    }

    #[test]
    fn rejects_empty_recovery_owner() {
        let data = build_script_args(b"jimleston_osoi", b"");

        let result = parse_script_args(&data);

        assert_eq!(result, Err(Error::EmptyRecoveryOwner));
    }

    #[test]
    fn rejects_duplicate_owner_roles() {
        let data =
            build_script_args(b"jimleston_osoi", b"jimleston_osoi");

        let result = parse_script_args(&data);

        assert_eq!(
            result,
            Err(Error::InvalidScriptArgsLength)
        );
    }

    #[test]
    fn parses_owner_authorization_witness() {
        let data =
            build_witness(ACTION_OWNER, 1, b"jimleston_osoi");

        let result =
            parse_authorization_witness(&data).unwrap();

        assert_eq!(result.version, CONTRACT_VERSION);
        assert_eq!(result.action, ACTION_OWNER);
        assert_eq!(result.nonce, 1);
        assert_eq!(result.identifier, b"jimleston_osoi");
    }

    #[test]
    fn parses_recovery_authorization_witness() {
        let data =
            build_witness(ACTION_RECOVERY, 20, b"recovery_admin");

        let result =
            parse_authorization_witness(&data).unwrap();

        assert_eq!(result.action, ACTION_RECOVERY);
        assert_eq!(result.nonce, 20);
        assert_eq!(result.identifier, b"recovery_admin");
    }

    #[test]
    fn rejects_zero_nonce() {
        let data =
            build_witness(ACTION_OWNER, 0, b"jimleston_osoi");

        let result = parse_authorization_witness(&data);

        assert_eq!(result, Err(Error::InvalidNonce));
    }

    #[test]
    fn rejects_unknown_action() {
        let data =
            build_witness(99, 1, b"jimleston_osoi");

        let result = parse_authorization_witness(&data);

        assert_eq!(result, Err(Error::UnsupportedAction));
    }

    #[test]
    fn accepts_primary_owner_authorization() {
        let configuration = LockConfiguration {
            version: CONTRACT_VERSION,
            primary_owner: b"jimleston_osoi".to_vec(),
            recovery_owner: b"recovery_admin".to_vec(),
        };

        let authorization = AuthorizationWitness {
            version: CONTRACT_VERSION,
            action: ACTION_OWNER,
            nonce: 1,
            identifier: b"jimleston_osoi".to_vec(),
        };

        let result =
            validate_authorization(&configuration, &authorization);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn rejects_wrong_primary_owner() {
        let configuration = LockConfiguration {
            version: CONTRACT_VERSION,
            primary_owner: b"jimleston_osoi".to_vec(),
            recovery_owner: b"recovery_admin".to_vec(),
        };

        let authorization = AuthorizationWitness {
            version: CONTRACT_VERSION,
            action: ACTION_OWNER,
            nonce: 1,
            identifier: b"another_owner".to_vec(),
        };

        let result =
            validate_authorization(&configuration, &authorization);

        assert_eq!(
            result,
            Err(Error::OwnerAuthorizationFailed)
        );
    }

    #[test]
    fn accepts_recovery_authorization() {
        let configuration = LockConfiguration {
            version: CONTRACT_VERSION,
            primary_owner: b"jimleston_osoi".to_vec(),
            recovery_owner: b"recovery_admin".to_vec(),
        };

        let authorization = AuthorizationWitness {
            version: CONTRACT_VERSION,
            action: ACTION_RECOVERY,
            nonce: 10,
            identifier: b"recovery_admin".to_vec(),
        };

        let result =
            validate_authorization(&configuration, &authorization);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn rejects_primary_owner_using_recovery_action() {
        let configuration = LockConfiguration {
            version: CONTRACT_VERSION,
            primary_owner: b"jimleston_osoi".to_vec(),
            recovery_owner: b"recovery_admin".to_vec(),
        };

        let authorization = AuthorizationWitness {
            version: CONTRACT_VERSION,
            action: ACTION_RECOVERY,
            nonce: 15,
            identifier: b"jimleston_osoi".to_vec(),
        };

        let result =
            validate_authorization(&configuration, &authorization);

        assert_eq!(
            result,
            Err(Error::RecoveryAuthorizationFailed)
        );
    }

    #[test]
    fn rejects_invalid_identifier_character() {
        let result =
            validate_identifier(b"jimleston@osoi");

        assert_eq!(
            result,
            Err(Error::InvalidOwnerCharacter)
        );
    }

    #[test]
    fn rejects_truncated_witness() {
        let data = [
            CONTRACT_VERSION,
            ACTION_OWNER,
            1,
            0,
            0,
        ];

        let result = parse_authorization_witness(&data);

        assert_eq!(
            result,
            Err(Error::InvalidWitnessLength)
        );
    }

    #[test]
    fn rejects_incorrect_identifier_length() {
        let mut data =
            build_witness(ACTION_OWNER, 1, b"jimleston_osoi");

        data[10] = 2;

        let result = parse_authorization_witness(&data);

        assert_eq!(
            result,
            Err(Error::IdentifierLengthMismatch)
        );
    }
}