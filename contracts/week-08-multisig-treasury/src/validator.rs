#![no_std]

use crate::{
    authorization::{
        verify_authorizations,
        AuthorizationSet,
    },
    config::TreasuryConfig,
    errors::Error,
};

pub fn validate_treasury(
    config_data: &[u8],
    witness_data: &[u8],
) -> Result<u8, Error> {
    let config =
        TreasuryConfig::parse(
            config_data,
        )?;

    let authorizations =
        AuthorizationSet::parse(
            witness_data,
        )?;

    verify_authorizations(
        &config,
        &authorizations,
    )
}s
