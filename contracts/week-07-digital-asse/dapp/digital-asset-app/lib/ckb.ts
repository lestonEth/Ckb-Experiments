import { ccc } from "@ckb-ccc/connector-react";

export const CKB_TESTNET_EXPLORER =
  "https://pudge.explorer.nervos.org";

export function explorerTransactionUrl(txHash: string) {
  return `${CKB_TESTNET_EXPLORER}/transaction/${txHash}`;
}

export function shortenHash(
  value: string,
  start = 10,
  end = 8,
) {
  if (!value) return "";

  if (value.length <= start + end) {
    return value;
  }

  return `${value.slice(0, start)}...${value.slice(-end)}`;
}

export function formatCkb(
  shannon: bigint | undefined,
) {
  if (shannon === undefined) {
    return "0";
  }

  return ccc.fixedPointToString(shannon);
}
