const ASSET_VERSION = 1;

export interface AssetData {
  version: number;
  assetId: Uint8Array;
  owner: Uint8Array;
  metadataHash: Uint8Array;
}

function hexToBytes(hex: string): Uint8Array {
  const normalized = hex.replace(/^0x/, "");

  if (normalized.length % 2 !== 0) {
    throw new Error("Invalid hex string");
  }

  const bytes = new Uint8Array(
    normalized.length / 2,
  );

  for (let i = 0; i < bytes.length; i++) {
    bytes[i] = parseInt(
      normalized.slice(i * 2, i * 2 + 2),
      16,
    );
  }

  return bytes;
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte) =>
      byte.toString(16).padStart(2, "0"),
    )
    .join("");
}

export function encodeAssetData(
  asset: AssetData,
): string {
  if (asset.assetId.length !== 32) {
    throw new Error(
      "Asset ID must contain 32 bytes",
    );
  }

  if (asset.owner.length !== 20) {
    throw new Error(
      "Owner must contain 20 bytes",
    );
  }

  if (asset.metadataHash.length !== 32) {
    throw new Error(
      "Metadata hash must contain 32 bytes",
    );
  }

  const result = new Uint8Array(85);

  result[0] = ASSET_VERSION;

  result.set(asset.assetId, 1);
  result.set(asset.owner, 33);
  result.set(asset.metadataHash, 53);

  return `0x${bytesToHex(result)}`;
}

export function decodeAssetData(
  data: string,
): AssetData {
  const bytes = hexToBytes(data);

  if (bytes.length !== 85) {
    throw new Error(
      "Invalid digital asset data length",
    );
  }

  if (bytes[0] !== ASSET_VERSION) {
    throw new Error(
      "Unsupported asset version",
    );
  }

  return {
    version: bytes[0],
    assetId: bytes.slice(1, 33),
    owner: bytes.slice(33, 53),
    metadataHash: bytes.slice(53, 85),
  };
}

export function bytesToHexString(
  bytes: Uint8Array,
) {
  return `0x${bytesToHex(bytes)}`;
}
