import { fiberRpc } from '../rpcClient.js';

import type {
    ListChannelsParams,
    ListChannelsResult,
} from '../types.js';

export async function listChannels(
    input: ListChannelsParams = {},
): Promise<ListChannelsResult> {
    if (
        input.include_closed &&
        input.only_pending
    ) {
        throw new Error(
            'include_closed and only_pending cannot both be true.',
        );
    }

    return fiberRpc.call<
        ListChannelsResult,
        Record<string, unknown>
    >(
        'list_channels',
        {
            pubkey: input.pubkey,
            include_closed:
                input.include_closed ?? false,
            only_pending:
                input.only_pending ?? false,
        },
    );
}