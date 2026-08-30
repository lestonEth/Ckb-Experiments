use crate::errors::AssetError;

pub const ASSET_VERSION: u8 = 1;

pub struct DigitalAsset {
    pub version: u8,
    pub asset_id: [u8; 32],
    pub owner: [u8; 20],
    pub metadata_hash: [u8; 32],
}

impl DigitalAsset {
    pub const SIZE: usize = 1 + 32 + 20 + 32;

    pub fn new(
        asset_id: [u8; 32],
        owner: [u8; 20],
        metadata_hash: [u8; 32],
    ) -> Self {
        Self {
            version: ASSET_VERSION,
            asset_id,
            owner,
            metadata_hash,
        }
    }

    pub fn encode(&self) -> [u8; Self::SIZE] {
        let mut data = [0u8; Self::SIZE];

        data[0] = self.version;

        data[1..33].copy_from_slice(
            &self.asset_id
        );

        data[33..53].copy_from_slice(
            &self.owner
        );

        data[53..85].copy_from_slice(
            &self.metadata_hash
        );

        data
    }

    pub fn decode(
        data: &[u8]
    ) -> Result<Self, AssetError> {
        if data.len() != Self::SIZE {
            return Err(
                AssetError::InvalidDataLength
            );
        }

        if data[0] != ASSET_VERSION {
            return Err(
                AssetError::InvalidVersion
            );
        }

        let mut asset_id = [0u8; 32];
        asset_id.copy_from_slice(
            &data[1..33]
        );

        let mut owner = [0u8; 20];
        owner.copy_from_slice(
            &data[33..53]
        );

        let mut metadata_hash = [0u8; 32];
        metadata_hash.copy_from_slice(
            &data[53..85]
        );

        Ok(Self {
            version: data[0],
            asset_id,
            owner,
            metadata_hash,
        })
    }
}
