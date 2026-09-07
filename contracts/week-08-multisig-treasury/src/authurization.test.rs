#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TreasuryConfig;

    fn signer(value: u8) -> [u8; 20] {
        [value; 20]
    }

    fn signature(value: u8) -> [u8; 32] {
        [value; 32]
    }

    fn config_bytes() -> std::vec::Vec<u8> {
        let mut data = std::vec::Vec::new();

        data.push(1);
        data.push(2);
        data.push(3);

        data.extend_from_slice(&signer(1));
        data.extend_from_slice(&signer(2));
        data.extend_from_slice(&signer(3));

        data
    }

    fn authorization_bytes(
        signers: &[[u8; 20]],
    ) -> std::vec::Vec<u8> {
        let mut data =
            std::vec::Vec::new();

        data.push(1);
        data.push(signers.len() as u8);

        for signer in signers {
            data.extend_from_slice(signer);
            data.extend_from_slice(
                &signature(9),
            );
        }

        data
    }

    #[test]
    fn accepts_two_valid_signers() {
        let config =
            TreasuryConfig::parse(
                &config_bytes(),
            )
            .unwrap();

        let witness =
            AuthorizationSet::parse(
                &authorization_bytes(
                    &[signer(1), signer(2)],
                ),
            )
            .unwrap();

        let result =
            verify_authorizations(
                &config,
                &witness,
            );

        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn accepts_three_valid_signers() {
        let config =
            TreasuryConfig::parse(
                &config_bytes(),
            )
            .unwrap();

        let witness =
            AuthorizationSet::parse(
                &authorization_bytes(
                    &[
                        signer(1),
                        signer(2),
                        signer(3),
                    ],
                ),
            )
            .unwrap();

        let result =
            verify_authorizations(
                &config,
                &witness,
            );

        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn rejects_single_signer() {
        let config =
            TreasuryConfig::parse(
                &config_bytes(),
            )
            .unwrap();

        let witness =
            AuthorizationSet::parse(
                &authorization_bytes(
                    &[signer(1)],
                ),
            )
            .unwrap();

        let result =
            verify_authorizations(
                &config,
                &witness,
            );

        assert_eq!(
            result,
            Err(
                Error::InsufficientApprovals
            )
        );
    }

    #[test]
    fn rejects_unknown_signer() {
        let config =
            TreasuryConfig::parse(
                &config_bytes(),
            )
            .unwrap();

        let witness =
            AuthorizationSet::parse(
                &authorization_bytes(
                    &[signer(1), signer(99)],
                ),
            )
            .unwrap();

        let result =
            verify_authorizations(
                &config,
                &witness,
            );

        assert_eq!(
            result,
            Err(Error::UnknownSigner)
        );
    }

    #[test]
    fn rejects_duplicate_signer() {
        let config =
            TreasuryConfig::parse(
                &config_bytes(),
            )
            .unwrap();

        let witness =
            AuthorizationSet::parse(
                &authorization_bytes(
                    &[signer(1), signer(1)],
                ),
            )
            .unwrap();

        let result =
            verify_authorizations(
                &config,
                &witness,
            );

        assert_eq!(
            result,
            Err(
                Error::DuplicateAuthorization
            )
        );
    }

    #[test]
    fn rejects_invalid_signature() {
        let config =
            TreasuryConfig::parse(
                &config_bytes(),
            )
            .unwrap();

        let mut data =
            std::vec::Vec::new();

        data.push(1);
        data.push(2);

        data.extend_from_slice(
            &signer(1),
        );

        data.extend_from_slice(
            &[0u8; 32],
        );

        data.extend_from_slice(
            &signer(2),
        );

        data.extend_from_slice(
            &signature(9),
        );

        let witness =
            AuthorizationSet::parse(
                &data,
            )
            .unwrap();

        let result =
            verify_authorizations(
                &config,
                &witness,
            );

        assert_eq!(
            result,
            Err(
                Error::InvalidAuthorization
            )
        );
    }

    #[test]
    fn rejects_invalid_threshold() {
        let mut data =
            std::vec::Vec::new();

        data.push(1);
        data.push(4);
        data.push(3);

        data.extend_from_slice(
            &signer(1),
        );

        data.extend_from_slice(
            &signer(2),
        );

        data.extend_from_slice(
            &signer(3),
        );

        assert_eq!(
            TreasuryConfig::parse(&data),
            Err(Error::InvalidThreshold)
        );
    }

    #[test]
    fn rejects_duplicate_config_signers() {
        let mut data =
            std::vec::Vec::new();

        data.push(1);
        data.push(2);
        data.push(3);

        data.extend_from_slice(
            &signer(1),
        );

        data.extend_from_slice(
            &signer(1),
        );

        data.extend_from_slice(
            &signer(3),
        );

        assert_eq!(
            TreasuryConfig::parse(&data),
            Err(Error::DuplicateSigner)
        );
    }
}
