import { fiberRpc } from '../rpcClient.js';

import type {
    ListPeersResult,
} from '../types.js';

export async function listPeers():
    Promise<ListPeersResult> {
    return fiberRpc.call<
        ListPeersResult,
        Record<string, never>
    >(
        'list_peers',
        {},
    );
}