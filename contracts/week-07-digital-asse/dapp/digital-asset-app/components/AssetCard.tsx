"use client";

import { DigitalAsset } from "@/lib/types";
import { shortenHash } from "@/lib/ckb";

interface AssetCardProps {
  asset: DigitalAsset;
  onTransfer?: () => void;
}

export function AssetCard({
  asset,
  onTransfer,
}: AssetCardProps) {
  return (
    <article
      className="overflow-hidden rounded-2xl
                 border border-zinc-800
                 bg-zinc-950"
    >
      <div className="h-40 bg-gradient-to-br
                      from-indigo-500/30
                      via-purple-500/20
                      to-cyan-500/20
                      p-6"
      >
        <div className="flex items-center justify-between">
          <span className="rounded-full
                           border border-white/10
                           bg-black/30 px-3 py-1
                           text-xs">
            DIGITAL ASSET
          </span>

          <span className="text-xs text-zinc-400">
            v{asset.version}
          </span>
        </div>

        <h3 className="mt-8 text-xl font-bold">
          {asset.metadata}
        </h3>
      </div>

      <div className="space-y-4 p-6">

        <div>
          <p className="text-xs text-zinc-500">
            Asset ID
          </p>

          <p className="mt-1 break-all font-mono text-sm">
            {shortenHash(asset.assetId)}
          </p>
        </div>

        <div>
          <p className="text-xs text-zinc-500">
            Owner
          </p>

          <p className="mt-1 break-all font-mono text-sm">
            {shortenHash(asset.owner, 14, 10)}
          </p>
        </div>

        {onTransfer && (
          <button
            onClick={onTransfer}
            className="w-full rounded-lg
                       bg-white px-4 py-3
                       text-sm font-semibold
                       text-black hover:bg-zinc-200"
          >
            Transfer Asset
          </button>
        )}

      </div>
    </article>
  );
}
