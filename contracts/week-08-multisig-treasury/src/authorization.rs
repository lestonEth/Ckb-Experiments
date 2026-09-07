#![no_std]

use crate::{
    config::{
        TreasuryConfig,
        SIGNER_ID_SIZE,
    },
    errors::Error,
};

pub const AUTH_VERSION: u8 = 1;
pub const SIGNATURE_SIZE: usize = 32;

pub const AUTHORIZATION_ENTRY_SIZE:
    usize = SIGNER_ID_SIZE + SIGNATURE_SIZE;

#[derive(Clone, Copy)]
pub struct Authorization {
    pub signer: [u8; SIGNER_ID_SIZE],
    pub signature: [u8; SIGNATURE_SIZE],
}

pub struct AuthorizationSet {
    pub version: u8,
    pub count: u8,
    pub authorizations:
        [Authorization; 3],
}

impl AuthorizationSet {
    pub fn parse(
        data: &[u8],
    ) -> Result<Self, Error> {
        if data.len() < 2 {
            return Err(
                Error::InvalidWitnessLength
            );
        }

        let version = data[0];

        if version != AUTH_VERSION {
            return Err(
                Error::InvalidWitnessVersion
            );
        }

        let count = data[1];

        if count == 0 || count > 3 {
            return Err(
                Error::InvalidSignatureCount
            );
        }

        let expected =
            2 + (
                count as usize
                * AUTHORIZATION_ENTRY_SIZE
            );

        if data.len() != expected {
            return Err(
                Error::InvalidWitnessLength
            );
        }

        let empty_authorization =
            Authorization {
                signer: [0u8; SIGNER_ID_SIZE],
                signature:
                    [0u8; SIGNATURE_SIZE],
            };

        let mut authorizations =
            [empty_authorization; 3];

        let mut i = 0;

        while i < count as usize {
            let offset =
                2 + (
                    i * AUTHORIZATION_ENTRY_SIZE
                );

            let signer_start = offset;

            let signer_end =
                signer_start + SIGNER_ID_SIZE;

            let signature_start =
                signer_end;

            let signature_end =
                signature_start + SIGNATURE_SIZE;

            let mut signer =
                [0u8; SIGNER_ID_SIZE];

            signer.copy_from_slice(
                &data[
                    signer_start..signer_end
                ],
            );

            let mut signature =
                [0u8; SIGNATURE_SIZE];

            signature.copy_from_slice(
                &data[
                    signature_start..
                    signature_end
                ],
            );

            authorizations[i] =
                Authorization {
                    signer,
                    signature,
                };

            i += 1;
        }

        Ok(Self {
            version,
            count,
            authorizations,
        })
    }
}

pub fn verify_authorizations(
    config: &TreasuryConfig,
    authorization_set: &AuthorizationSet,
) -> Result<u8, Error> {
    let mut valid_count = 0;

    let mut i = 0;

    while i < authorization_set.count as usize {
        let authorization =
            &authorization_set.authorizations[i];

        // Every signer must belong to the
        // configured signer set.
        if !config.contains_signer(
            &authorization.signer,
        ) {
            return Err(
                Error::UnknownSigner
            );
        }

        // Prevent duplicate signer entries.
        let mut previous = 0;

        while previous < i {
            if authorization_set
                .authorizations[previous]
                .signer
                == authorization.signer
            {
                return Err(
                    Error::DuplicateAuthorization
                );
            }

            previous += 1;
        }

        if !valid_signature(
            authorization,
        ) {
            return Err(
                Error::InvalidAuthorization
            );
        }

        valid_count += 1;

        i += 1;
    }

    if valid_count < config.threshold {
        return Err(
            Error::InsufficientApprovals
        );
    }

    Ok(valid_count)
}

fn valid_signature(
    authorization: &Authorization,
) -> bool {
    /*
     * Educational signature validation.
     *
     * A real implementation should perform
     * cryptographic verification against the
     * transaction message.
     *
     * For this learning contract we require
     * a non-zero proof.
     */
    authorization
        .signature
        .iter()
        .any(|byte| *byte != 0)
}
