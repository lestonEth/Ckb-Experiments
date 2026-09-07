"use client";

import { useEffect, useState } from "react";
import { ccc } from "@ckb-ccc/connector-react";

import { formatCkb } from "@/lib/ckb";

export function Dashboard() {
  const signer: any = ccc.useSigner();

  const [address, setAddress] =
    useState<string>("");

  const [balance, setBalance] =
    useState<bigint>();

  useEffect(() => {
    if (!signer) {
      setAddress("");
      setBalance(undefined);
      return;
    }

    let cancelled = false;

    async function loadWallet() {
      const [
        recommendedAddress,
        walletBalance,
      ] = await Promise.all([
        signer.getRecommendedAddress(),
        signer.getBalance(),
      ]);

      if (cancelled) return;

      setAddress(recommendedAddress);
      setBalance(walletBalance);
    }

    loadWallet();

    return () => {
      cancelled = true;
    };
  }, [signer]);

  if (!signer) {
    return (
      <div className="rounded-2xl border border-zinc-800
                      bg-zinc-950 p-8 text-center">
        <h2 className="text-xl font-semibold">
          Connect your wallet
        </h2>

        <p className="mt-2 text-zinc-500">
          Connect a CKB wallet to manage digital assets.
        </p>
      </div>
    );
  }

  return (
    <section className="grid gap-4 md:grid-cols-2">

      <div className="rounded-2xl border border-zinc-800
                      bg-zinc-950 p-6">
        <p className="text-sm text-zinc-500">
          Connected wallet
        </p>

        <p className="mt-3 break-all text-sm">
          {address}
        </p>
      </div>

      <div className="rounded-2xl border border-zinc-800
                      bg-zinc-950 p-6">
        <p className="text-sm text-zinc-500">
          Testnet balance
        </p>

        <p className="mt-3 text-2xl font-bold">
          {formatCkb(balance)} CKB
        </p>
      </div>

    </section>
  );
}
