"use client";

import { useState } from "react";
import { ccc } from "@ckb-ccc/connector-react";

import {
  DigitalAsset,
} from "@/lib/types";

interface TransferAssetProps {
  asset: DigitalAsset;
  onClose?: () => void;
}

export function TransferAsset({
  asset,
  onClose,
}: TransferAssetProps) {
  const signer = ccc.useSigner();

  const [recipient, setRecipient] =
    useState("");

  const [loading, setLoading] =
    useState(false);

  const [error, setError] =
    useState<string | null>(null);

  const [txHash, setTxHash] =
    useState<string | null>(null);

  async function transfer() {
    if (!signer) {
      setError(
        "Connect your wallet first.",
      );
      return;
    }

    if (!recipient.trim()) {
      setError(
        "Enter a CKB address.",
      );
      return;
    }

    try {
      setLoading(true);
      setError(null);

      const {
        script: recipientLock,
      } = await ccc.Address.fromString(
        recipient.trim(),
        signer.client,
      );

      /*
       * Educational transaction:
       *
       * The complete implementation will locate
       * the asset Cell by its type script,
       * consume that Cell and recreate it with
       * the recipient's ownership lock.
       *
       * For now we demonstrate the transaction
       * construction layer using the recipient
       * lock.
       */

      const tx = ccc.Transaction.from({
        outputs: [
          {
            lock: recipientLock,
            capacity: ccc.fixedPointFrom("62"),
          },
        ],
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

    } catch (err) {
      console.error(err);

      setError(
        err instanceof Error
          ? err.message
          : "Transfer failed",
      );
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="rounded-2xl border
                    border-zinc-800
                    bg-zinc-950 p-6">

      <div className="flex items-center
                      justify-between">

        <div>
          <h2 className="text-xl font-semibold">
            Transfer Asset
          </h2>

          <p className="mt-1 text-sm text-zinc-500">
            Transfer ownership to another CKB address.
          </p>
        </div>

        {onClose && (
          <button
            onClick={onClose}
            className="text-sm text-zinc-500
                       hover:text-white"
          >
            Close
          </button>
        )}

      </div>

      <div className="mt-6">

        <label className="text-sm text-zinc-400">
          Recipient CKB Address
        </label>

        <textarea
          value={recipient}
          onChange={(e) =>
            setRecipient(e.target.value)
          }
          rows={3}
          placeholder="ckt1..."
          className="mt-2 w-full rounded-lg
                     border border-zinc-800
                     bg-zinc-900 px-4 py-3
                     font-mono text-sm
                     outline-none
                     focus:border-zinc-600"
        />

        <button
          onClick={transfer}
          disabled={loading}
          className="mt-4 w-full rounded-lg
                     bg-white px-4 py-3
                     text-sm font-semibold
                     text-black
                     disabled:opacity-50"
        >
          {loading
            ? "Signing transaction..."
            : "Transfer Ownership"}
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
            Transaction submitted.
          </p>

          <p className="mt-2 break-all
                        font-mono text-xs
                        text-zinc-400">
            {txHash}
          </p>
        </div>
      )}

    </div>
  );
}
