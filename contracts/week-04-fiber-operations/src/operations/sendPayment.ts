import { fiberRpc } from '../rpcClient.js';
import { toRpcHex } from '../utils.js';

import type {
    PaymentResult,
} from '../types.js';

export interface PayInvoiceInput {
    invoice: string;
    timeoutSeconds?: bigint;
    maxFeeAmount?: bigint;
    maxFeeRate?: bigint;
    maxParts?: bigint;
    dryRun?: boolean;
}

export async function sendPayment(
    input: PayInvoiceInput,
): Promise<PaymentResult> {
    if (!input.invoice.trim()) {
        throw new Error(
            'Invoice address cannot be empty.',
        );
    }

    return fiberRpc.call<
        PaymentResult,
        Record<string, unknown>
    >(
        'send_payment',
        {
            invoice: input.invoice.trim(),

            timeout:
                input.timeoutSeconds === undefined
                    ? undefined
                    : toRpcHex(
                        input.timeoutSeconds,
                    ),

            max_fee_amount:
                input.maxFeeAmount === undefined
                    ? undefined
                    : toRpcHex(
                        input.maxFeeAmount,
                    ),

            max_fee_rate:
                input.maxFeeRate === undefined
                    ? undefined
                    : toRpcHex(
                        input.maxFeeRate,
                    ),

            max_parts:
                input.maxParts === undefined
                    ? undefined
                    : toRpcHex(
                        input.maxParts,
                    ),

            dry_run: input.dryRun ?? false,
        },
    );
}