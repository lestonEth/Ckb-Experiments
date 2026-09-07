"use client";

import { useState } from "react";
import { ccc } from "@ckb-ccc/connector-react";

import {
  encodeAssetData,
} from "@/lib/asset";

interface CreateAssetProps {
  onCreated?: (txHash: string) => void;
}

function randomBytes(length: number) {
  const bytes = new Uint8Array(length);

  crypto.getRandomValues(bytes);

  return bytes;
}

export function CreateAsset({
  onCreated,
}: CreateAssetProps) {
  const signer = ccc.useSigner();

  const [name, setName] =
    useState("CKB Learning Asset");

  const [metadata, setMetadata] =
    useState("Week 7 Digital Asset");

  const [loading, setLoading] =
    useState(false);

  const [error, setError] =
    useState<string | null>(null);

  const [txHash, setTxHash] =
    useState<string | null>(null);

  async function createAsset() {
    if (!signer) {
      setError("Connect your wallet first.");
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setTxHash(null);

      const ownerAddress =
        await signer.getRecommendedAddress();

      const { script: ownerLock } =
        await ccc.Address.fromString(
          ownerAddress,
          signer.client,
        );

      const assetId = randomBytes(32);
      const metadataHash = randomBytes(32);

      const data = encodeAssetData({
        version: 1,
        assetId,
        owner: new Uint8Array(20),
        metadataHash,
      });

      /*
       * This creates a normal CKB Cell containing
       * our digital asset data.
       *
       * The deployed custom type script will later
       * be added here as a CellDep + output type.
       */

      const tx = ccc.Transaction.from({
        outputs: [
          {
            lock: ownerLock,
            capacity: ccc.fixedPointFrom("62"),
          },
        ],

        outputsData: [data],
      });

      await tx.completeInputsByCapacity(
        signer,
      );

      await tx.completeFeeBy(
        signer,
      );

      const hash =
        await signer.sendTransaction(tx);

      setTxHash(hash);

      onCreated?.(hash);

    } catch (err) {
      console.error(err);

      setError(
        err instanceof Error
          ? err.message
          : "Failed to create asset",
      );
    } finally {
      setLoading(false);
    }
  }

  return (
    <section className="rounded-2xl border
                        border-zinc-800
                        bg-zinc-950 p-6">

      <div>
        <h2 className="text-xl font-semibold">
          Create Digital Asset
        </h2>

        <p className="mt-1 text-sm text-zinc-500">
          Create an asset Cell on CKB Testnet.
        </p>
      </div>

      <div className="mt-6 space-y-4">

        <input
          value={name}
          onChange={(e) =>
            setName(e.target.value)
          }
          placeholder="Asset name"
          className="w-full rounded-lg
                     border border-zinc-800
                     bg-zinc-900 px-4 py-3
                     outline-none
                     focus:border-zinc-600"
        />

        <textarea
          value={metadata}
          onChange={(e) =>
            setMetadata(e.target.value)
          }
          placeholder="Metadata"
          rows={3}
          className="w-full rounded-lg
                     border border-zinc-800
                     bg-zinc-900 px-4 py-3
                     outline-none
                     focus:border-zinc-600"
        />

        <button
          onClick={createAsset}
          disabled={loading}
          className="w-full rounded-lg
                     bg-white px-4 py-3
                     text-sm font-semibold
                     text-black
                     disabled:cursor-not-allowed
                     disabled:opacity-50"
        >
          {loading
            ? "Creating..."
            : "Create Digital Asset"}
        </button>

      </div>

      {error && (
        <div className="mt-4 rounded-lg
                        border border-red-900
                        bg-red-950/30
                        p-4 text-sm text-red-300">
          {error}
        </div>
      )}

      {txHash && (
        <div className="mt-4 rounded-lg
                        border border-green-900
                        bg-green-950/30
                        p-4">

          <p className="text-sm text-green-300">
            Asset transaction submitted.
          </p>

          <p className="mt-2 break-all font-mono
                        text-xs text-zinc-400">
            {txHash}
          </p>
        </div>
      )}

    </section>
  );
}
