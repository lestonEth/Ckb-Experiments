import { fiberRpc } from '../rpcClient.js';

import type {
    ParseInvoiceResult,
} from '../types.js';

export async function parseInvoice(
    invoice: string,
): Promise<ParseInvoiceResult> {
    if (!invoice.trim()) {
        throw new Error(
            'Invoice address cannot be empty.',
        );
    }

    return fiberRpc.call<
        ParseInvoiceResult,
        Record<string, unknown>
    >(
        'parse_invoice',
        {
            invoice: invoice.trim(),
        },
    );
}