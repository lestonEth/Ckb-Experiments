use crate::{
    asset::DigitalAsset,
    errors::AssetError,
};

pub fn validate_transfer(
    input: &DigitalAsset,
    output: &DigitalAsset,
) -> Result<(), AssetError> {

    if input.asset_id != output.asset_id {
        return Err(
            AssetError::AssetIdChanged
        );
    }

    if input.metadata_hash != output.metadata_hash {
        return Err(
            AssetError::MetadataChanged
        );
    }

    if output.version != input.version {
        return Err(
            AssetError::InvalidVersion
        );
    }

    if input.owner == output.owner {
        return Err(
            AssetError::InvalidOwnershipTransfer
        );
    }

    Ok(())
}
