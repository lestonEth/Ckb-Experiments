# Week 4: Fiber Payment Operations Toolkit

## CKBuilder Program

**Author:** Jimleston Osoi
**Track:** Nervos CKB and Fiber Development
**Language:** TypeScript
**Project:** Fiber peer, channel, invoice and payment operations

---

## Overview

This project is a TypeScript command-line toolkit for interacting with a Fiber Network Node through JSON-RPC.

The project demonstrates a basic Fiber payment workflow:

```text
Connect to a peer
        ↓
List connected peers
        ↓
Inspect payment channels
        ↓
Create an invoice
        ↓
Parse the invoice
        ↓
Send a payment
        ↓
Retrieve payment status
        ↓
List payment history
```

The toolkit separates every Fiber operation into an individual TypeScript module. This makes the code easier to understand, test and update as the Fiber RPC API evolves.

---

## Implemented Operations

The project includes the following Fiber RPC operations:

| File              | RPC method      | Purpose                        |
| ----------------- | --------------- | ------------------------------ |
| `connectPeer.ts`  | `connect_peer`  | Connect to another Fiber node  |
| `listPeers.ts`    | `list_peers`    | List connected peers           |
| `listChannels.ts` | `list_channels` | Inspect local payment channels |
| `newInvoice.ts`   | `new_invoice`   | Generate a payment invoice     |
| `parseInvoice.ts` | `parse_invoice` | Decode an invoice              |
| `sendPayment.ts`  | `send_payment`  | Send an invoice payment        |
| `getPayment.ts`   | `get_payment`   | Retrieve one payment           |
| `listPayments.ts` | `list_payments` | List outgoing payments         |

---

## Project Structure

```text
week-04-fiber-payments/
├── .env.example
├── package.json
├── tsconfig.json
├── README.md
└── src/
    ├── config.ts
    ├── index.ts
    ├── rpcClient.ts
    ├── types.ts
    ├── utils.ts
    └── operations/
        ├── connectPeer.ts
        ├── getPayment.ts
        ├── listChannels.ts
        ├── listPayments.ts
        ├── listPeers.ts
        ├── newInvoice.ts
        ├── parseInvoice.ts
        └── sendPayment.ts
```

---

## Requirements

Before running the project, install:

* Node.js 20 or newer
* npm or pnpm
* A running Fiber Network Node
* Access to the Fiber node JSON-RPC address
* At least one connected Fiber peer
* A usable payment channel for actual payments

---

## Installation

Clone or enter the project directory:

```bash
cd week-04-fiber-payments
```

Install the dependencies:

```bash
npm install
```

Create the environment file:

```bash
cp .env.example .env
```

---

## Environment Configuration

Configure the Fiber node RPC address:

```env
FIBER_RPC_URL=http://127.0.0.1:8227
FIBER_CURRENCY=Fibt
FIBER_REQUEST_TIMEOUT_MS=30000
```

Available invoice currencies are:

```text
Fibb = CKB mainnet
Fibt = CKB testnet
Fibd = CKB devnet
```

Do not expose the Fiber JSON-RPC port publicly. Restrict access to the local machine or other trusted systems.

---

## Type Checking

Run TypeScript validation:

```bash
npm run check
```

Build the project:

```bash
npm run build
```

---

## CLI Commands

Show the available commands:

```bash
npm run fiber -- help
```

### Connect using a multi-address

```bash
npm run fiber -- connect-peer-address \
  /ip4/127.0.0.1/tcp/8228/p2p/PEER_ID
```

### Connect using a public key

```bash
npm run fiber -- connect-peer-pubkey \
  02aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
```

The node must already know an address for the public key through its network graph.

### List connected peers

```bash
npm run fiber -- peers
```

### List all active channels

```bash
npm run fiber -- channels
```

### List channels associated with one peer

```bash
npm run fiber -- channels-for \
  02aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
```

### List pending channel operations

```bash
npm run fiber -- pending-channels
```

### Create an invoice

The amount is supplied in shannons.

```bash
npm run fiber -- new-invoice \
  100000000 \
  "CKBuilder Week 4 payment"
```

`100000000` shannons represents 1 CKB.

The command returns an encoded invoice address:

```text
Invoice address:
fibt1...
```

### Parse an invoice

```bash
npm run fiber -- parse-invoice \
  "fibt1..."
```

This displays the invoice currency, amount, payment hash, description, expiry and related attributes.

### Test payment routing without sending

```bash
npm run fiber -- dry-run-payment \
  "fibt1..."
```

The dry-run option asks the Fiber node to check route availability and expected fees without completing the payment.

### Send a payment

```bash
npm run fiber -- pay \
  "fibt1..."
```

The result may contain:

```text
payment_hash
payment_preimage
status
created_at
last_updated_at
fee
failed_error
routers
```

### Retrieve a payment

```bash
npm run fiber -- payment \
  0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
```

### List payments

List all recent payments:

```bash
npm run fiber -- payments
```

List successful payments:

```bash
npm run fiber -- payments Success 20
```

Supported payment status values are:

```text
Created
Inflight
Success
Failed
```

---

## Example Workflow

### Terminal 1: Receiving Node

Create an invoice:

```bash
npm run fiber -- new-invoice \
  100000000 \
  "Payment from Week 4 sender"
```

Copy the returned invoice address.

### Terminal 2: Sending Node

Inspect the invoice:

```bash
npm run fiber -- parse-invoice \
  "fibt1..."
```

Perform a route dry run:

```bash
npm run fiber -- dry-run-payment \
  "fibt1..."
```

Send the payment:

```bash
npm run fiber -- pay \
  "fibt1..."
```

Copy the returned payment hash and inspect the payment:

```bash
npm run fiber -- payment \
  0x...
```

---

## Error Handling

The toolkit handles:

* Invalid Fiber RPC URLs
* Request timeouts
* JSON-RPC errors
* HTTP failures
* Missing CLI arguments
* Invalid public keys
* Invalid payment hashes
* Invalid payment statuses
* Invalid numeric values
* Conflicting channel filters
* Empty invoices

Errors are printed to the terminal and the process exits with a failure code.

---

## Security

The toolkit is intended for learning and local development.

Important precautions:

* Do not expose the Fiber RPC port to the public internet.
* Do not commit private keys or node secrets.
* Use testnet or devnet during development.
* Confirm invoice amounts before paying.
* Use `dry-run-payment` before sending unfamiliar invoices.
* Verify that the channel has sufficient outbound liquidity.
* Review fee limits before sending large payments.

---

## Learning Outcomes

During Week 4, I learned how to:

* Communicate with a Fiber Network Node using JSON-RPC.
* Connect to Fiber peers.
* Inspect connected peers and payment channels.
* Create and parse Fiber invoices.
* Send invoice-based payments.
* Check payment state.
* Work with payment hashes.
* Apply payment fee limits.
* Use dry-run payment routing.
* Handle RPC failures in TypeScript.
* Structure Fiber operations into reusable modules.

---

## Future Improvements

Possible future additions include:

* `disconnect_peer`
* `open_channel`
* `accept_channel`
* `shutdown_channel`
* `get_invoice`
* `cancel_invoice`
* `settle_invoice`
* Payment pagination
* Channel opening progress monitoring
* Keysend payments
* Multi-part payments
* Trampoline routing
* Channel rebalancing
* Unit and integration tests
* Interactive CLI prompts
* Web dashboard integration

---

## Week 4 Conclusion

This project introduced practical Fiber Network operations without requiring direct implementation of payment-channel protocol internals.

The toolkit covers the main payment lifecycle: connecting to peers, inspecting channels, creating invoices, parsing invoices, sending payments and tracking payment results.

It provides a reusable foundation for building more advanced Fiber tools, wallets, payment dashboards and merchant applications.
