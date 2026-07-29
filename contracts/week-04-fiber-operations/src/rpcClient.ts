import { config } from './config.js';

import type {
    JsonRpcRequest,
    JsonRpcResponse,
} from './types.js';

export class FiberRpcError extends Error {
    constructor(
        message: string,
        public readonly code?: number,
        public readonly data?: unknown,
    ) {
        super(message);
        this.name = 'FiberRpcError';
    }
}

export class FiberRpcClient {
    private requestId = 0;

    constructor(
        private readonly rpcUrl: string =
            config.rpcUrl,
        private readonly timeoutMs: number =
            config.requestTimeoutMs,
    ) { }

    async call<
        TResult,
        TParams extends Record<string, unknown>,
    >(
        method: string,
        params: TParams,
    ): Promise<TResult> {
        const id = ++this.requestId;

        const request: JsonRpcRequest<TParams> = {
            jsonrpc: '2.0',
            id,
            method,
            params,
        };

        const controller = new AbortController();

        const timeout = setTimeout(
            () => controller.abort(),
            this.timeoutMs,
        );

        try {
            const response = await fetch(
                this.rpcUrl,
                {
                    method: 'POST',
                    headers: {
                        'content-type': 'application/json',
                    },
                    body: JSON.stringify(request),
                    signal: controller.signal,
                },
            );

            if (!response.ok) {
                throw new FiberRpcError(
                    `Fiber RPC returned HTTP ${response.status}.`,
                );
            }

            const payload =
                (await response.json()) as
                JsonRpcResponse<TResult>;

            if ('error' in payload) {
                throw new FiberRpcError(
                    payload.error.message,
                    payload.error.code,
                    payload.error.data,
                );
            }

            return payload.result;
        } catch (error) {
            if (
                error instanceof DOMException &&
                error.name === 'AbortError'
            ) {
                throw new FiberRpcError(
                    `Fiber RPC request timed out after ${this.timeoutMs}ms.`,
                );
            }

            if (error instanceof FiberRpcError) {
                throw error;
            }

            throw new FiberRpcError(
                error instanceof Error
                    ? error.message
                    : 'Unknown Fiber RPC error.',
            );
        } finally {
            clearTimeout(timeout);
        }
    }
}

export const fiberRpc =
    new FiberRpcClient();