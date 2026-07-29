export function toRpcHex(
    value: string | number | bigint,
): string {
    const amount =
        typeof value === 'bigint'
            ? value
            : BigInt(value);

    if (amount < 0n) {
        throw new Error(
            'Numeric RPC values cannot be negative.',
        );
    }

    return `0x${amount.toString(16)}`;
}

export function parsePositiveInteger(
    value: string,
    fieldName: string,
): bigint {
    if (!/^\d+$/.test(value)) {
        throw new Error(
            `${fieldName} must be a positive decimal integer.`,
        );
    }

    const parsed = BigInt(value);

    if (parsed <= 0n) {
        throw new Error(
            `${fieldName} must be greater than zero.`,
        );
    }

    return parsed;
}

export function assertHash256(
    value: string,
    fieldName = 'payment hash',
): string {
    const normalised = value.startsWith('0x')
        ? value
        : `0x${value}`;

    if (!/^0x[0-9a-fA-F]{64}$/.test(normalised)) {
        throw new Error(
            `${fieldName} must be a 32-byte hexadecimal value.`,
        );
    }

    return normalised.toLowerCase();
}

export function assertPubkey(
    value: string,
): string {
    const normalised = value.startsWith('0x')
        ? value.slice(2)
        : value;

    if (!/^[0-9a-fA-F]{66}$/.test(normalised)) {
        throw new Error(
            'Peer public key must be a 33-byte compressed public key.',
        );
    }

    return normalised.toLowerCase();
}

export function printJson(
    value: unknown,
): void {
    console.log(
        JSON.stringify(value, null, 2),
    );
}