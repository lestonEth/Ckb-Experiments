import { connectPeer } from './operations/connectPeer.js';
import { getPayment } from './operations/getPayment.js';
import { listChannels } from './operations/listChannels.js';
import { listPayments } from './operations/listPayments.js';
import { listPeers } from './operations/listPeers.js';
import { newInvoice } from './operations/newInvoice.js';
import { parseInvoice } from './operations/parseInvoice.js';
import { sendPayment } from './operations/sendPayment.js';

import {
    assertPubkey,
    parsePositiveInteger,
    printJson,
} from './utils.js';

import type {
    PaymentStatus,
} from './types.js';

function printHelp(): void {
    console.log(`
Fiber Payment Operations Toolkit

Usage:
  npm run fiber -- connect-peer-address <multiaddr>
  npm run fiber -- connect-peer-pubkey <pubkey>
  npm run fiber -- peers
  npm run fiber -- channels
  npm run fiber -- channels-for <pubkey>
  npm run fiber -- pending-channels
  npm run fiber -- new-invoice <amount-shannons> [description]
  npm run fiber -- parse-invoice <invoice>
  npm run fiber -- pay <invoice>
  npm run fiber -- dry-run-payment <invoice>
  npm run fiber -- payment <payment-hash>
  npm run fiber -- payments [status] [limit]

Payment statuses:
  Created
  Inflight
  Success
  Failed
`);
}

function requireArgument(
    value: string | undefined,
    name: string,
): string {
    if (!value?.trim()) {
        throw new Error(
            `Missing required argument: ${name}.`,
        );
    }

    return value.trim();
}

function parsePaymentStatus(
    value: string | undefined,
): PaymentStatus | undefined {
    if (!value) {
        return undefined;
    }

    const statuses: PaymentStatus[] = [
        'Created',
        'Inflight',
        'Success',
        'Failed',
    ];

    const status = statuses.find(
        item =>
            item.toLowerCase() ===
            value.toLowerCase(),
    );

    if (!status) {
        throw new Error(
            `Invalid payment status: ${value}.`,
        );
    }

    return status;
}

async function main(): Promise<void> {
    const [
        command,
        first,
        second,
    ] = process.argv.slice(2);

    switch (command) {
        case 'connect-peer-address': {
            const address = requireArgument(
                first,
                'peer multi-address',
            );

            await connectPeer({
                address,
                save: true,
            });

            console.log(
                'Peer connected successfully.',
            );

            return;
        }

        case 'connect-peer-pubkey': {
            const pubkey = assertPubkey(
                requireArgument(
                    first,
                    'peer public key',
                ),
            );

            await connectPeer({
                pubkey,
                save: true,
                addr_type: 'tcp',
            });

            console.log(
                'Peer connected successfully.',
            );

            return;
        }

        case 'peers': {
            printJson(
                await listPeers(),
            );

            return;
        }

        case 'channels': {
            printJson(
                await listChannels(),
            );

            return;
        }

        case 'channels-for': {
            const pubkey = assertPubkey(
                requireArgument(
                    first,
                    'peer public key',
                ),
            );

            printJson(
                await listChannels({
                    pubkey,
                }),
            );

            return;
        }

        case 'pending-channels': {
            printJson(
                await listChannels({
                    only_pending: true,
                }),
            );

            return;
        }

        case 'new-invoice': {
            const amount = parsePositiveInteger(
                requireArgument(
                    first,
                    'amount in shannons',
                ),
                'Invoice amount',
            );

            const result = await newInvoice({
                amount,
                description:
                    second ??
                    'CKBuilder Week 4 Fiber payment',

                expirySeconds: 3_600n,
                allowMpp: false,
            });

            printJson(result);

            console.log(
                '\nInvoice address:\n',
                result.invoice_address,
            );

            return;
        }

        case 'parse-invoice': {
            const invoice = requireArgument(
                first,
                'invoice',
            );

            printJson(
                await parseInvoice(invoice),
            );

            return;
        }

        case 'pay': {
            const invoice = requireArgument(
                first,
                'invoice',
            );

            printJson(
                await sendPayment({
                    invoice,
                    timeoutSeconds: 60n,
                    maxFeeRate: 5n,
                }),
            );

            return;
        }

        case 'dry-run-payment': {
            const invoice = requireArgument(
                first,
                'invoice',
            );

            printJson(
                await sendPayment({
                    invoice,
                    timeoutSeconds: 60n,
                    maxFeeRate: 5n,
                    dryRun: true,
                }),
            );

            return;
        }

        case 'payment': {
            const paymentHash =
                requireArgument(
                    first,
                    'payment hash',
                );

            printJson(
                await getPayment(paymentHash),
            );

            return;
        }

        case 'payments': {
            const status =
                parsePaymentStatus(first);

            const limit =
                second === undefined
                    ? 15n
                    : parsePositiveInteger(
                        second,
                        'Payment limit',
                    );

            printJson(
                await listPayments({
                    status,
                    limit,
                }),
            );

            return;
        }

        case 'help':
        case '--help':
        case '-h':
        case undefined:
            printHelp();
            return;

        default:
            throw new Error(
                `Unknown command: ${command}`,
            );
    }
}

main().catch(error => {
    console.error(
        '\nFiber operation failed:',
    );

    console.error(
        error instanceof Error
            ? error.message
            : error,
    );

    process.exitCode = 1;
});