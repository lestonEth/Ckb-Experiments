#[derive(Debug, PartialEq, Eq)]
pub enum AssetError {
    InvalidDataLength,
    InvalidVersion,
    AssetIdChanged,
    MetadataChanged,
    InvalidOwnershipTransfer,
    DuplicateAsset,
    OwnerNotAuthorized,
}
