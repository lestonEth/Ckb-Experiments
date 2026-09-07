#![no_std]

use crate::errors::Error;

pub const VERSION: u8 = 1;
pub const SIGNER_ID_SIZE: usize = 20;
pub const MAX_SIGNERS: usize = 3;

#[derive(Clone, Copy)]
pub struct TreasuryConfig {
    pub version: u8,
    pub threshold: u8,
    pub signer_count: u8,
    pub signers: [[u8; SIGNER_ID_SIZE]; MAX_SIGNERS],
}

impl TreasuryConfig {
    pub fn parse(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 3 {
            return Err(Error::InvalidConfigLength);
        }

        let version = data[0];

        if version != VERSION {
            return Err(Error::UnsupportedVersion);
        }

        let threshold = data[1];
        let signer_count = data[2];

        if signer_count == 0 || signer_count > MAX_SIGNERS as u8 {
            return Err(Error::InvalidSignerCount);
        }

        if threshold == 0 || threshold > signer_count {
            return Err(Error::InvalidThreshold);
        }

        let expected_length =
            3 + (signer_count as usize * SIGNER_ID_SIZE);

        if data.len() != expected_length {
            return Err(Error::InvalidConfigLength);
        }

        let mut signers =
            [[0u8; SIGNER_ID_SIZE]; MAX_SIGNERS];

        let mut i = 0;

        while i < signer_count as usize {
            let start =
                3 + (i * SIGNER_ID_SIZE);

            let end =
                start + SIGNER_ID_SIZE;

            let mut signer =
                [0u8; SIGNER_ID_SIZE];

            signer.copy_from_slice(
                &data[start..end],
            );

            if signer.iter().all(|x| *x == 0) {
                return Err(Error::EmptySigner);
            }

            // Reject duplicate signer identifiers.
            let mut previous = 0;

            while previous < i {
                if signers[previous] == signer {
                    return Err(Error::DuplicateSigner);
                }

                previous += 1;
            }

            signers[i] = signer;

            i += 1;
        }

        Ok(Self {
            version,
            threshold,
            signer_count,
            signers,
        })
    }

    pub fn contains_signer(
        &self,
        signer: &[u8; SIGNER_ID_SIZE],
    ) -> bool {
        let mut i = 0;

        while i < self.signer_count as usize {
            if self.signers[i] == *signer {
                return true;
            }

            i += 1;
        }

        false
    }
}
