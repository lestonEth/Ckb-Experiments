# CKBuilder Program

# Week 4 Technical Journal

## Advanced Fiber Operations and Payment Integration for Corven

**Prepared by:** Jimleston Osoi
**Programme:** CKBuilder Programme – Nervos CKB Developer Track
**Week:** 4
**Primary Technology:** TypeScript
**Project:** Corven Cloud Development Environment

---

# Executive Summary

During Week 4 of the CKBuilder Programme, I expanded my work from CKB smart-contract development into practical Fiber Network operations.

The primary deliverable for this week was a modular TypeScript toolkit that communicates with a running Fiber Network Node through JSON-RPC. The toolkit supports essential peer, channel, invoice, and payment operations, including connecting to peers, inspecting channels, generating invoices, sending payments, and retrieving payment status.

This work is directly connected to **Corven**, the browser-based development environment I am building for CKB and Fiber developers.

Corven is intended to remove the complexity of local blockchain development by providing preconfigured workspaces, integrated tooling, persistent storage, terminal access, automated runtime provisioning, contract compilation, testing utilities, and Fiber node management from a single web interface.

The Week 4 Fiber toolkit will serve as the foundation for Corven’s future **Fiber Operations Dashboard**, where developers will be able to manage peers, inspect channels, generate invoices, and test payments without manually writing JSON-RPC commands.

---

# Project Context: Corven

Corven is a cloud-based integrated development environment designed specifically for Nervos CKB and Fiber development.

The platform aims to provide developers with a ready-to-use environment that includes:

* Rust and RISC-V toolchains
* CKB contract templates
* Integrated terminal access
* File management
* Contract compilation
* CKB debugger integration
* Mock transaction testing
* Persistent cloud workspaces
* Automated Fiber node provisioning
* Peer and channel management
* Invoice and payment testing

The long-term objective is to enable developers to build, test, debug, and operate CKB and Fiber applications entirely from the browser.

The Week 4 project contributes specifically to the Fiber operations layer of Corven.

---

# Week 4 Objectives

The main objectives for Week 4 were to:

* Understand how applications communicate with Fiber nodes.
* Study the Fiber JSON-RPC interface.
* Implement reusable TypeScript RPC operations.
* Connect a local Fiber node to remote peers.
* Inspect connected peers.
* Inspect active and pending payment channels.
* Generate Fiber invoices.
* Parse and validate invoice information.
* Send invoice-based payments.
* Check individual payment records.
* List previous payments.
* Handle RPC errors and request timeouts.
* Prepare Fiber functionality for integration into Corven.

---

# Project Overview

The Week 4 project is named:

```text
week-04-fiber-payments
```

It is a TypeScript toolkit for interacting with a Fiber Network Node through JSON-RPC.

The toolkit currently implements the following operations:

```text
connect_peer
list_peers
list_channels
new_invoice
parse_invoice
send_payment
get_payment
list_payments
```

These operations cover the basic lifecycle of a Fiber payment:

```text
Connect to a Fiber peer
          |
          v
Inspect connected peers
          |
          v
Inspect payment channels
          |
          v
Generate an invoice
          |
          v
Parse and verify the invoice
          |
          v
Send a payment
          |
          v
Retrieve payment status
          |
          v
Review payment history
```

---

# Repository Structure

The project follows a modular TypeScript architecture:

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

Each Fiber operation is stored in a separate module.

This structure improves:

* Maintainability
* Testability
* Reusability
* Error isolation
* Future API migration
* Integration with Corven microservices

---

# Understanding Fiber Network

Fiber Network is a peer-to-peer payment network designed to support fast and low-cost transactions through payment channels.

Rather than recording every payment directly on the base blockchain, users can exchange payments through off-chain channels and settle final state changes when required.

Fiber supports capabilities such as:

* Fast payments
* Low transaction costs
* Multi-hop routing
* Payment-channel liquidity
* Invoice-based payments
* Multiple assets
* Peer-to-peer connectivity
* Scalable payment applications

A Fiber node is responsible for managing:

* Peer connections
* Payment channels
* Network routes
* Invoice creation
* Payment execution
* Payment status
* Channel liquidity
* Node-level RPC operations

---

# JSON-RPC Client Architecture

The toolkit communicates with the Fiber node using JSON-RPC 2.0.

A standard RPC request follows this structure:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "list_peers",
  "params": {}
}
```

A successful response contains:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {}
}
```

A failed request contains an RPC error:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -1,
    "message": "RPC operation failed"
  }
}
```

The central `FiberRpcClient` is responsible for:

* Generating unique request identifiers
* Sending HTTP requests
* Applying request timeouts
* Parsing JSON responses
* Detecting RPC failures
* Converting failures into reusable TypeScript errors
* Returning strongly typed results

Centralising RPC communication prevents repeated networking logic across the operation modules.

---

# Environment Configuration

The project uses environment variables for node configuration.

```env
FIBER_RPC_URL=http://127.0.0.1:8227
FIBER_CURRENCY=Fibt
FIBER_REQUEST_TIMEOUT_MS=30000
```

The configuration determines:

* The Fiber node RPC address
* The invoice network
* The RPC timeout duration

Supported invoice currencies include:

```text
Fibb = CKB mainnet
Fibt = CKB testnet
Fibd = CKB devnet
```

During development, testnet or devnet should be used to reduce risk and simplify testing.

---

# Peer Management

## Connecting to a Peer

The `connectPeer.ts` operation implements:

```text
connect_peer
```

It supports connection using either:

* A peer multi-address
* A compressed peer public key

Example:

```bash
npm run fiber -- connect-peer-address \
  /ip4/127.0.0.1/tcp/8228/p2p/PEER_ID
```

The application validates the supplied arguments before sending the RPC request.

This operation is important because a Fiber node must connect to other nodes before discovering routes or participating in payment-channel activity.

## Listing Connected Peers

The `listPeers.ts` module implements:

```text
list_peers
```

It retrieves currently connected nodes and displays useful fields such as:

```text
pubkey
address
```

Within Corven, this data can be displayed in a peer management table showing:

* Peer public key
* Network address
* Connection status
* Connection timestamp
* Available actions

---

# Channel Operations

The `listChannels.ts` module implements:

```text
list_channels
```

It supports:

* Listing active channels
* Filtering channels by peer
* Including closed channels
* Displaying pending channels

Useful channel fields may include:

```text
channel_id
peer_pubkey
state_name
local_balance
remote_balance
is_public
is_acceptor
```

The local balance represents the outbound liquidity controlled by the local node.

The remote balance represents liquidity controlled by the connected peer.

Before sending a payment, the node should have:

* An active channel
* Sufficient outbound capacity
* A valid route
* A reachable destination

In Corven, this information can be displayed in a channel dashboard with visual status indicators.

---

# Invoice Operations

## Creating an Invoice

The `newInvoice.ts` module implements:

```text
new_invoice
```

It accepts values such as:

* Amount
* Description
* Currency
* Expiry
* Multi-part payment configuration
* Trampoline-routing configuration

Example:

```bash
npm run fiber -- new-invoice \
  100000000 \
  "CKBuilder Week 4 payment"
```

The amount is supplied in shannons.

The operation returns an encoded invoice address:

```text
fibt1...
```

Within Corven, this operation can be exposed through an invoice creation form containing:

* Amount
* Description
* Expiry
* Currency
* Routing options

The generated invoice can then be displayed with copy and QR-code actions.

## Parsing an Invoice

The `parseInvoice.ts` module implements:

```text
parse_invoice
```

It allows a developer to inspect invoice data before sending payment.

The parsed result may include:

* Currency
* Amount
* Payment hash
* Description
* Expiry
* Payee information
* Invoice attributes

Example:

```bash
npm run fiber -- parse-invoice "fibt1..."
```

Within Corven, this functionality can power an invoice inspector that verifies payment information before submission.

---

# Payment Operations

## Sending a Payment

The `sendPayment.ts` operation implements:

```text
send_payment
```

The current implementation supports invoice-based payments.

Example:

```bash
npm run fiber -- pay "fibt1..."
```

The request can include:

* Payment timeout
* Maximum fee amount
* Maximum fee rate
* Maximum number of payment parts
* Dry-run mode

The response may contain:

```text
payment_hash
payment_preimage
status
fee
created_at
last_updated_at
failed_error
routers
```

This information is essential for debugging and monitoring payment execution.

## Dry-Run Payments

The toolkit supports payment simulation:

```bash
npm run fiber -- dry-run-payment "fibt1..."
```

Dry-run mode can be used to:

* Check route availability
* Estimate fees
* Detect insufficient liquidity
* Validate an invoice
* Avoid accidental payment execution

This is particularly useful inside Corven because developers can inspect the expected payment path before sending funds.

## Retrieving Payment Status

The `getPayment.ts` operation implements:

```text
get_payment
```

It retrieves a payment using its payment hash.

Example:

```bash
npm run fiber -- payment 0x...
```

The result provides information such as:

* Current payment status
* Payment preimage
* Failure reason
* Routing information
* Fee paid
* Creation time
* Last update time

## Listing Payments

The `listPayments.ts` module implements:

```text
list_payments
```

It can list outgoing payments and filter them by status.

Supported statuses include:

```text
Created
Inflight
Success
Failed
```

Example:

```bash
npm run fiber -- payments Success 20
```

Within Corven, this data can be displayed as a searchable payment history table.

---

# Integration with Corven

The Week 4 toolkit is not an isolated learning project. It is intended to become part of Corven’s Fiber management functionality.

Corven currently uses a microservice-oriented architecture that includes:

* API Gateway
* Authentication Service
* Workspace Service
* Runtime Service
* Storage Service
* Template Service
* Future Fiber Node Service

The Fiber toolkit can be integrated into a dedicated service:

```text
fiber-service
```

The proposed architecture is:

```text
Corven Web Application
          |
          v
API Gateway
          |
          v
Fiber Service
          |
          v
Fiber JSON-RPC Client
          |
          v
Workspace Fiber Node
```

The frontend would never communicate directly with the Fiber RPC port.

Instead, requests would pass through Corven’s authenticated backend.

This provides:

* Access control
* Request validation
* User isolation
* Workspace isolation
* Audit logging
* Error handling
* Secure RPC access
* Consistent API responses

---

# Proposed Corven API Endpoints

The Week 4 operation modules can be exposed through Corven endpoints such as:

```text
POST /api/fiber/peers/connect
GET  /api/fiber/peers
GET  /api/fiber/channels
POST /api/fiber/invoices
POST /api/fiber/invoices/parse
POST /api/fiber/payments
GET  /api/fiber/payments/:paymentHash
GET  /api/fiber/payments
```

Example request:

```http
POST /api/fiber/invoices
Content-Type: application/json
Authorization: Bearer <token>
```

```json
{
  "workspaceId": "workspace-id",
  "amount": "100000000",
  "description": "Corven test payment"
}
```

The Fiber Service would resolve the correct workspace node and forward the request to its RPC interface.

---

# Proposed Corven User Interface

The Fiber toolkit can support several Corven dashboard views.

## Peer Management

The peer interface may include:

* Connected peers
* Peer public keys
* Network addresses
* Connect peer action
* Disconnect peer action
* Connection status

## Channel Management

The channel dashboard may include:

* Channel ID
* Peer
* Channel state
* Local balance
* Remote balance
* Pending status
* Open and close actions

## Invoice Management

The invoice view may include:

* Invoice creation form
* Amount
* Description
* Expiry
* Generated invoice string
* QR code
* Copy action
* Parsed invoice details

## Payment Dashboard

The payment interface may include:

* Invoice input
* Dry-run action
* Route summary
* Maximum fee
* Payment status
* Payment history
* Failure details
* Payment hash
* Payment preimage

---

# Security Considerations

The Fiber RPC interface should not be exposed directly to public users.

Corven should protect the Fiber node through its backend services.

Security controls should include:

* Authentication
* Workspace ownership checks
* RPC access restrictions
* Input validation
* Payment amount limits
* Fee limits
* Request rate limiting
* Audit logs
* Testnet defaults
* Confirmation before payment
* Isolation between user workspaces

Private keys, node secrets, payment preimages, and sensitive configuration should never be exposed through frontend logs.

---

# Error Handling

The toolkit handles several failure conditions:

* Invalid RPC URL
* Request timeout
* HTTP failure
* JSON-RPC error
* Invalid public key
* Invalid payment hash
* Invalid amount
* Missing invoice
* Invalid payment status
* Missing command arguments
* Unsupported channel filters
* Route failure
* Insufficient liquidity

Errors are converted into clear TypeScript exceptions and displayed through the command-line interface.

When integrated into Corven, these errors can be mapped to structured API responses and user-friendly interface notifications.

---

# Challenges Encountered

## Understanding Fiber RPC Structure

The first challenge was understanding how Fiber operations are organised across peer, channel, invoice, and payment modules.

Separating each RPC operation into an individual TypeScript file made the workflow easier to understand.

## Handling Hexadecimal RPC Values

Several Fiber RPC fields use hexadecimal values rather than ordinary decimal numbers.

I created reusable conversion utilities to safely transform decimal values into RPC-compatible hexadecimal strings.

## Validating Public Keys and Hashes

Peer public keys and payment hashes require exact byte lengths.

Validation was added before requests are sent to prevent malformed RPC calls.

## Designing for Corven Integration

The toolkit initially worked as a standalone CLI.

I then structured it so the same functions could later be imported into Corven’s backend Fiber Service without rewriting the RPC logic.

---

# Key Learning Outcomes

By completing Week 4, I learned how to:

* Communicate with Fiber nodes using JSON-RPC.
* Structure a reusable TypeScript RPC client.
* Connect to Fiber peers.
* Inspect channels and liquidity.
* Create and parse invoices.
* Send invoice-based payments.
* Check payment status.
* Use dry-run routing.
* Validate public keys and payment hashes.
* Handle RPC errors safely.
* Separate CLI logic from reusable service logic.
* Design Fiber operations for integration into Corven.

---

# Relationship to Previous Weeks

The first four weeks now form a clear development progression.

## Week 1

I studied:

* CKB architecture
* The Cell Model
* Transaction structure
* Rust development setup
* RISC-V contract compilation

## Week 2

I built a Profile Cell Type Script that validates:

* Profile format
* Cell creation
* Cell updates
* Output count
* Profile data constraints

## Week 3

I built a multi-role Lock Script supporting:

* Primary owner authorization
* Recovery owner authorization
* Script arguments
* Witness parsing
* Versioning
* Action codes
* Nonce validation

## Week 4

I moved into Fiber application development and implemented:

* Peer management
* Channel inspection
* Invoice creation
* Invoice parsing
* Payment execution
* Payment monitoring
* Corven integration planning

This progression covers both on-chain CKB contract development and off-chain Fiber payment operations.

---

# Future Development

The next stage is to integrate these operation modules into the Corven backend.

Planned improvements include:

* Create a dedicated Fiber microservice.
* Provision one Fiber node per workspace.
* Store workspace-specific RPC configuration.
* Add peer connection endpoints.
* Add channel opening and shutdown operations.
* Add invoice cancellation and settlement.
* Stream payment status updates.
* Add channel liquidity monitoring.
* Add transaction and payment logs.
* Build a Fiber operations dashboard.
* Add QR-code invoice generation.
* Add integration tests with multiple local Fiber nodes.
* Add automated workspace network setup.

---

# Conclusion

Week 4 marked the transition from CKB contract development into practical Fiber Network application development.

The Fiber Payment Operations Toolkit provides reusable TypeScript modules for peer connections, channel inspection, invoice generation, invoice parsing, payment execution, and payment monitoring.

The work directly supports Corven’s objective of becoming a complete browser-based environment for CKB and Fiber developers.

Rather than requiring developers to manually manage terminals, RPC commands, node configuration, and payment requests, Corven will eventually provide these capabilities through an integrated and secure user interface.

This Week 4 project therefore represents both a CKBuilder learning milestone and an important technical foundation for Corven’s Fiber development platform.
