import { config } from '../config.js';
import { fiberRpc } from '../rpcClient.js';
import { toRpcHex } from '../utils.js';

import type {
    NewInvoiceResult,
} from '../types.js';

export interface CreateInvoiceInput {
    amount: bigint;
    description?: string;
    expirySeconds?: bigint;
    allowMpp?: boolean;
    allowTrampolineRouting?: boolean;
}

export async function newInvoice(
    input: CreateInvoiceInput,
): Promise<NewInvoiceResult> {
    if (input.amount <= 0n) {
        throw new Error(
            'Invoice amount must be greater than zero.',
        );
    }

    return fiberRpc.call<
        NewInvoiceResult,
        Record<string, unknown>
    >(
        'new_invoice',
        {
            amount: toRpcHex(input.amount),
            description:
                input.description ??
                'CKBuilder Week 4 Fiber payment',

            currency: config.currency,

            expiry:
                input.expirySeconds === undefined
                    ? undefined
                    : toRpcHex(
                        input.expirySeconds,
                    ),

            allow_mpp:
                input.allowMpp ?? false,

            allow_trampoline_routing:
                input.allowTrampolineRouting ??
                false,
        },
    );
}