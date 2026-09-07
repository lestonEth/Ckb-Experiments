"use client";

import { useState } from "react";

import ConnectWallet from "@/components/ConnectWallet"
import { Dashboard } from "@/components/Dashboard";
import { CreateAsset } from "@/components/CreateAsset";
import { AssetCard } from "@/components/AssetCard";
import { TransferAsset } from "@/components/TransferAsset";

import {
  DigitalAsset,
} from "@/lib/types";

const demoAsset: DigitalAsset = {
  version: 1,
  assetId:
    "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  owner:
    "0x1234567890abcdef1234567890abcdef",
  metadata:
    "CKB Learning Asset #001",
};

export default function Home() {
  const [asset] =
    useState<DigitalAsset | null>(
      demoAsset,
    );

  const [showTransfer, setShowTransfer] =
    useState(false);

  const [refreshKey, setRefreshKey] =
    useState(0);

  return (
    <main className="min-h-screen
                    bg-black text-white">

      <header className="border-b
                         border-zinc-900">

        <div className="mx-auto flex max-w-6xl
                        items-center justify-between
                        px-6 py-5">

          <div>
            <p className="text-xs
                          uppercase tracking-[0.3em]
                          text-zinc-500">
              Week 07
            </p>

            <h1 className="mt-1 text-xl
                           font-bold">
              CKB Digital Asset Lab
            </h1>
          </div>

          <ConnectWallet />

        </div>

      </header>

      <div className="mx-auto max-w-6xl
                      px-6 py-10">

        <section className="max-w-3xl">

          <span className="rounded-full
                           border border-indigo-500/20
                           bg-indigo-500/10
                           px-3 py-1
                           text-xs text-indigo-300">
            Nervos CKB
          </span>

          <h2 className="mt-5 text-4xl
                         font-bold tracking-tight
                         md:text-6xl">
            Digital Asset
            <span className="text-zinc-500">
              {" "}Ownership
            </span>
          </h2>

          <p className="mt-5 text-lg
                        leading-8 text-zinc-500">
            A learning dApp exploring how
            digital asset ownership can be
            represented and transferred using
            CKB Cells and Rust scripts.
          </p>

        </section>

        <div className="mt-12">
          <Dashboard />
        </div>

        <section className="mt-8 grid
                            gap-8 lg:grid-cols-2">

          <CreateAsset
            onCreated={() =>
              setRefreshKey(
                (value) => value + 1,
              )
            }
          />

          {asset && !showTransfer && (
            <AssetCard
              key={refreshKey}
              asset={asset}
              onTransfer={() =>
                setShowTransfer(true)
              }
            />
          )}

          {asset && showTransfer && (
            <TransferAsset
              asset={asset}
              onClose={() =>
                setShowTransfer(false)
              }
            />
          )}

        </section>

        <section className="mt-12
                            rounded-2xl
                            border border-zinc-800
                            bg-zinc-950 p-8">

          <h2 className="text-xl font-semibold">
            Ownership Model
          </h2>

          <div className="mt-6 grid gap-6
                          md:grid-cols-3">

            <ModelStep
              number="01"
              title="Create"
              description="Create an asset Cell containing its immutable identity and metadata."
            />

            <ModelStep
              number="02"
              title="Transfer"
              description="Consume the previous Cell and create a new ownership state."
            />

            <ModelStep
              number="03"
              title="Validate"
              description="The Rust script checks that the asset identity and metadata remain unchanged."
            />

          </div>

        </section>

      </div>

    </main>
  );
}

function ModelStep({
  number,
  title,
  description,
}: {
  number: string;
  title: string;
  description: string;
}) {
  return (
    <div>
      <span className="text-xs
                       font-mono text-indigo-400">
        {number}
      </span>

      <h3 className="mt-2 text-lg
                     font-semibold">
        {title}
      </h3>

      <p className="mt-2 text-sm
                    leading-6 text-zinc-500">
        {description}
      </p>
    </div>
  );
}
