import { fiberRpc } from '../rpcClient.js';
import {
    assertHash256,
    toRpcHex,
} from '../utils.js';

import type {
    ListPaymentsResult,
    PaymentStatus,
} from '../types.js';

export interface PaymentListInput {
    status?: PaymentStatus;
    limit?: bigint;
    after?: string;
}

export async function listPayments(
    input: PaymentListInput = {},
): Promise<ListPaymentsResult> {
    if (
        input.limit !== undefined &&
        input.limit <= 0n
    ) {
        throw new Error(
            'Payment list limit must be greater than zero.',
        );
    }

    return fiberRpc.call<
        ListPaymentsResult,
        Record<string, unknown>
    >(
        'list_payments',
        {
            status: input.status,

            limit:
                input.limit === undefined
                    ? undefined
                    : toRpcHex(input.limit),

            after:
                input.after === undefined
                    ? undefined
                    : assertHash256(
                        input.after,
                        'payment cursor',
                    ),
        },
    );
}