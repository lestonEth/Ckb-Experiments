export interface DigitalAsset {
  version: number;
  assetId: string;
  owner: string;
  metadata: string;
  metadataHash?: string;
  txHash?: string;
}

export interface AssetTransferResult {
  txHash: string;
  assetId: string;
  from: string;
  to: string;
}
