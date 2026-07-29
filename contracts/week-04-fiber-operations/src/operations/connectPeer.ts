import { fiberRpc } from '../rpcClient.js';

import type {
    ConnectPeerParams,
} from '../types.js';

export async function connectPeer(
    input: ConnectPeerParams,
): Promise<void> {
    if (!input.address && !input.pubkey) {
        throw new Error(
            'Provide either a peer address or public key.',
        );
    }

    await fiberRpc.call<
        null,
        Record<string, unknown>
    >(
        'connect_peer',
        {
            address: input.address,
            pubkey: input.pubkey,
            save: input.save ?? true,
            addr_type: input.addr_type,
        },
    );
}