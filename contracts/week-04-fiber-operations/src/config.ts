import 'dotenv/config';

import type { FiberCurrency } from './types.js';

function getRequiredUrl(
    value: string | undefined,
    fallback: string,
): string {
    const resolved = value?.trim() || fallback;

    try {
        return new URL(resolved).toString();
    } catch {
        throw new Error(
            `Invalid FIBER_RPC_URL: "${resolved}".`,
        );
    }
}

function getPositiveInteger(
    value: string | undefined,
    fallback: number,
): number {
    if (!value) {
        return fallback;
    }

    const parsed = Number(value);

    if (!Number.isInteger(parsed) || parsed <= 0) {
        throw new Error(
            'FIBER_REQUEST_TIMEOUT_MS must be a positive integer.',
        );
    }

    return parsed;
}

function getCurrency(
    value: string | undefined,
): FiberCurrency {
    const currency = value || 'Fibt';

    if (
        currency !== 'Fibb' &&
        currency !== 'Fibt' &&
        currency !== 'Fibd'
    ) {
        throw new Error(
            'FIBER_CURRENCY must be Fibb, Fibt, or Fibd.',
        );
    }

    return currency;
}

export const config = {
    rpcUrl: getRequiredUrl(
        process.env.FIBER_RPC_URL,
        'http://127.0.0.1:8227',
    ),

    currency: getCurrency(
        process.env.FIBER_CURRENCY,
    ),

    requestTimeoutMs: getPositiveInteger(
        process.env.FIBER_REQUEST_TIMEOUT_MS,
        30_000,
    ),
};