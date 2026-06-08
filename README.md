# Indexer

A high-performance blockchain indexing service that synchronizes on-chain ERC20 event data, decodes smart contract events, and provides fast, queryable access through a GraphQL API.

## Overview

The indexer connects to an EVM-compatible chain via WebSocket RPC, listens for new blocks in real time, decodes ERC20 `Transfer` and `Approval` events, and persists both raw and decoded data to PostgreSQL. A GraphQL API built on `async-graphql` and `Axum` exposes the indexed data for querying.

## Architecture

```
EVM RPC (wss://...)
        │
        ▼
  WS Listener (tokio task)
  Subscribes to new blocks, spawns a task per block
        │
        ▼
  Analyzer
  Fetches logs via eth_getLogs, decodes ERC20 events
        │
   ┌────┴────┐
   ▼         ▼
PostgreSQL  Token Metadata
Raw logs,   Fetched from contract on first seen
Transfers,  (name, symbol, decimals)
Approvals,
Tokens
        │
        ▼
  GraphQL API (Axum)
  /graphql + /graphql/playground
```

## Project Structure

```
indexer/
├── Cargo.toml
├── .env
├── docker-compose.yml
├── db/
│   └── migrations/
│       ├── 001_create_tokens.sql
│       ├── 002_create_raw_logs.sql
│       ├── 003_create_erc20_transfers.sql
│       └── 004_create_erc20_approvals.sql
├── abi/
│   └── IERC20.json
└── src/
    ├── main.rs                  — boots Axum, wires all layers
    ├── config.rs                — loads env config
    ├── error.rs                 — unified AppError type
    ├── types.rs                 — shared row types (RawLogRow, TransferRow, etc.)
    ├── provider/
    │   └── connect.rs           — HTTP and WSS provider setup
    ├── indexer/
    │   ├── mod.rs               — starts the WS listener task
    │   └── listener.rs          — block subscription loop
    │   └── analyzer.rs               — per-block log fetching, decoding, saving
    ├── decoder/
    │   └── decode.rs            — ERC20 event decoding via alloy sol! macro
    ├── tokens/
    │   └── fetch.rs             — fetches token metadata from contract
    ├── db/
    │   ├── raw_logs.rs          — insert + query raw logs
    │   ├── transfers.rs         — insert + query ERC20 transfers
    │   ├── approvals.rs         — insert + query ERC20 approvals
    │   └── tokens.rs            — upsert + query token metadata
    ├── graphql/
    │   ├── schema.rs            — builds the merged QueryRoot schema
    │   ├── models/
    │   │   ├── raw_log.rs       — RawLog GraphQL object
    │   │   ├── transfers.rs     — Transfer GraphQL object
    │   │   ├── approvals.rs     — Approval GraphQL object
    │   │   └── token.rs         — Token GraphQL object
    │   └── resolvers/
    │       ├── token.rs         — token, tokens queries
    │       ├── transfers.rs     — transfer, transfers queries
    │       ├── approvals.rs     — approval, approvals queries
    │       └── raw_logs.rs      — log, logs queries
    └── utils/
        └── contracts.rs         — alloy sol! macro for IERC20
```

## Prerequisites

- Rust 1.86+
- Docker + Docker Compose
- An EVM RPC provider with HTTP and WebSocket URLs (e.g. Alchemy, Infura)

## Setup

### 1. Clone the repository

```bash
git clone https://github.com/harwarl/indexer.git
cd indexer
```

### 2. Configure environment variables

Copy the example env file and fill in your values:

```bash
cp .env.example .env
```

```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/indexer
RPC_URL=https://base-mainnet.g.alchemy.com/v2/<your-key>
RPC_URL_WSS=wss://base-mainnet.g.alchemy.com/v2/<your-key>
RUST_LOG=info
```

### 3. Start the database

```bash
docker compose up -d
```

### 4. Run migrations

```bash
sqlx migrate run --source db/migrations --database-url postgresql://postgres:postgres@localhost:5432/indexer
```

### 5. Run the indexer

```bash
cargo run
```

The server starts on `http://0.0.0.0:3000`.

## Environment Variables

| Variable       | Required | Description                                   |
| -------------- | -------- | --------------------------------------------- |
| `DATABASE_URL` | Yes      | PostgreSQL connection string                  |
| `RPC_URL`      | Yes      | HTTP RPC endpoint for log fetching            |
| `RPC_URL_WSS`  | Yes      | WebSocket RPC endpoint for block subscription |
| `RUST_LOG`     | No       | Log level (default: `info`)                   |

## GraphQL API

The GraphQL playground is available at:

```
http://localhost:3000/graphql/playground
```

### Queries

#### Tokens

```graphql
# Get a single token by contract address
query {
  token(address: "0x...") {
    address
    name
    symbol
    decimals
    firstSeenBlock
    lastSeenBlock
  }
}

# List all indexed tokens
query {
  tokens(limit: 20, offset: 0) {
    address
    symbol
    decimals
  }
}
```

#### Transfers

```graphql
# List transfers with optional filters
query {
  transfers(
    tokenAddress: "0x..."
    fromAddress: "0x..."
    toAddress: "0x..."
    fromBlock: 1000000
    toBlock: 2000000
    limit: 20
    offset: 0
  ) {
    txHash
    logIndex
    blockNumber
    blockTimestamp
    address
    fromAddress
    toAddress
    value
  }
}

# Get a single transfer by tx hash and log index
query {
  transfer(txHash: "0x...", logIndex: 0) {
    txHash
    fromAddress
    toAddress
    value
  }
}
```

#### Approvals

```graphql
# List approvals with optional filters
query {
  approvals(
    tokenAddress: "0x..."
    ownerAddress: "0x..."
    spenderAddress: "0x..."
    fromBlock: 1000000
    limit: 20
  ) {
    txHash
    logIndex
    address
    owner
    spender
    value
    blockNumber
  }
}

# Get a single approval
query {
  approval(txHash: "0x...", logIndex: 0) {
    owner
    spender
    value
  }
}
```

#### Raw Logs

```graphql
# List raw logs with optional filters
query {
  logs(address: "0x...", txHash: "0x...", fromBlock: 1000000, limit: 20) {
    blockNumber
    blockTimestamp
    txHash
    logIndex
    address
    topics
    data
  }
}

# Get a single raw log
query {
  log(txHash: "0x...", logIndex: 0) {
    topics
    data
    address
  }
}
```

## Database Schema

| Table             | Description                                      |
| ----------------- | ------------------------------------------------ |
| `tokens`          | ERC20 token metadata (name, symbol, decimals)    |
| `raw_logs`        | Raw on-chain logs exactly as returned by the RPC |
| `erc20_transfers` | Decoded ERC20 Transfer events                    |
| `erc20_approvals` | Decoded ERC20 Approval events                    |

## Key Dependencies

| Crate           | Purpose                        |
| --------------- | ------------------------------ |
| `alloy`         | EVM RPC provider, ABI decoding |
| `axum`          | HTTP server                    |
| `async-graphql` | GraphQL schema and execution   |
| `sqlx`          | Async PostgreSQL queries       |
| `tokio`         | Async runtime                  |
| `tracing`       | Structured logging             |

## License

MIT
