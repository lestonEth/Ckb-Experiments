export type FiberCurrency = 'Fibb' | 'Fibt' | 'Fibd';

export type PaymentStatus =
    | 'Created'
    | 'Inflight'
    | 'Success'
    | 'Failed';

export interface JsonRpcRequest<TParams> {
    jsonrpc: '2.0';
    id: number;
    method: string;
    params: TParams;
}

export interface JsonRpcSuccess<TResult> {
    jsonrpc: '2.0';
    id: number;
    result: TResult;
}

export interface JsonRpcFailure {
    jsonrpc: '2.0';
    id: number;
    error: {
        code: number;
        message: string;
        data?: unknown;
    };
}

export type JsonRpcResponse<TResult> =
    | JsonRpcSuccess<TResult>
    | JsonRpcFailure;

export interface PeerInfo {
    pubkey: string;
    address: string;
}

export interface ListPeersResult {
    peers: PeerInfo[];
}

export interface FiberChannel {
    channel_id: string;
    is_public?: boolean;
    is_acceptor?: boolean;
    is_one_way?: boolean;
    channel_outpoint?: unknown;
    peer_pubkey?: string;
    state_name?: string;
    local_balance?: string;
    remote_balance?: string;
    [key: string]: unknown;
}

export interface ListChannelsResult {
    channels: FiberChannel[];
}

export interface InvoiceData {
    timestamp?: string;
    payment_hash?: string;
    attrs?: unknown[];
    [key: string]: unknown;
}

export interface CkbInvoice {
    currency?: FiberCurrency;
    amount?: string;
    signature?: string;
    data?: InvoiceData;
    [key: string]: unknown;
}

export interface NewInvoiceResult {
    invoice_address: string;
    invoice: CkbInvoice;
}

export interface ParseInvoiceResult {
    invoice: CkbInvoice;
}

export interface PaymentResult {
    payment_hash: string;
    payment_preimage?: string | null;
    status: PaymentStatus;
    created_at?: string;
    last_updated_at?: string;
    failed_error?: string | null;
    fee?: string;
    custom_records?: Record<string, string> | null;
    routers?: unknown[];
}

export interface ListPaymentsResult {
    payments: PaymentResult[];
    last_cursor?: string | null;
}

export interface ConnectPeerParams {
    address?: string;
    pubkey?: string;
    save?: boolean;
    addr_type?: 'tcp' | 'ws' | 'wss';
}

export interface ListChannelsParams {
    pubkey?: string;
    include_closed?: boolean;
    only_pending?: boolean;
}

export interface NewInvoiceParams {
    amount: string;
    description?: string;
    currency: FiberCurrency;
    payment_preimage?: string;
    payment_hash?: string;
    expiry?: string;
    fallback_address?: string;
    final_expiry_delta?: string;
    allow_mpp?: boolean;
    allow_trampoline_routing?: boolean;
}

export interface SendPaymentParams {
    invoice?: string;
    target_pubkey?: string;
    amount?: string;
    payment_hash?: string;
    final_tlc_expiry_delta?: string;
    tlc_expiry_limit?: string;
    timeout?: string;
    max_fee_amount?: string;
    max_fee_rate?: string;
    max_parts?: string;
    keysend?: boolean;
    allow_self_payment?: boolean;
    dry_run?: boolean;
}

export interface ListPaymentsParams {
    status?: PaymentStatus;
    limit?: string;
    after?: string;
}