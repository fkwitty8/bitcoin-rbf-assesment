
### PROJECT STRUCTURE
```
rbf_assesment/
├── Cargo.toml
├── .env.example
├── .gitignore
├── README.md
│
└── src/
|   ├── main.rs                     # Entry point (initializes logger, config, container & dispatches CLI)
|   │
|   ├── core/                       # CROSS-CUTTING / SYSTEM CORE
|   │   ├── mod.rs                  # Core module exports
|   │   ├── container.rs            # Dependency Injection (DI) Composition Root
|   │   ├── exceptions.rs                # AppError enum (combines Domain, RPC, Config & Infra errors)
|   │   └── logger.rs               # Tracing-subscriber / logging initialization
|   │
|   ├── domain/                     # 1. DOMAIN LAYER (Enterprise / Core Business Rules)
|   │   ├── mod.rs                  # Domain module exports
|   │   │
|   │   ├── entities/               # Complex domain objects with identity
|   │   │   ├── mod.rs
|   │   │   ├── blockchain.rs       # ChainState entity (Chain, Blocks, Headers)
|   │   │   └── wallet.rs           # Wallet entity (Name, TxCount)
|   │   │
|   │   ├── value_objects/          # Immutable domain value objects
|   │   │   ├── mod.rs
|   │   │   ├── address.rs          # Bitcoin Address VO (validates string format)
|   │   │   ├── balance.rs          # Satoshi / BTC Balance VO (prevents negative values)
|   │   │   └── difficulty.rs       # Chain difficulty VO
|   │   │
|   │   ├── enums/                  # Pure Domain Enums
|   │   │   ├── mod.rs
|   │   │   └── address_type_enums.rs     # AddressType (Bech32, Legacy, P2shSegwit, Bech32m)
|   │   │
|   │   ├── ports/                  # Outbound Interfaces / Traits (Dependency Inversion)
|   │   │   ├── mod.rs
|   │   │   ├── blockchain_port.rs  # Trait for fetching blockchain state
|   │   │   ├── wallet_port.rs      # Trait for wallet operations (balance, address, info)
|   │   │   └── raw_rpc_port.rs     # Trait for executing generic JSON-RPC methods
|   │   │
|   │   └── exceptions.rs               # DomainError enum (pure business failure cases)
|   │
|   ├── application/                # 2. APPLICATION LAYER (Use Cases / Orchestration)
|   │   ├── mod.rs                  # Application module exports
|   │   │
|   │   ├── dtos/                   # Application Data Transfer Objects (Input/Output commands)
|   │   │   ├── mod.rs
|   │   │   ├── blockchain_dto.rs   # Output DTO for blockchain summary
|   │   │   └── wallet_dto.rs       # Output DTO for wallet summary
|   │   │
|   │   └── use_cases/              # Independent application use cases
|   │       ├── mod.rs
|   │       ├── wallet_usecase # wallet usecases
|   |       |   ├── mod.rs
|   |       |   ├── get_wallet_info_usecase.rs     # Retrieves target wallet information
|   |       |   ├── get_wallet_balance_usecase.rs         # Obtains current wallet balance
|   |       |   ├── generate_wallet_address_usecase.rs    # Generates a new Bitcoin address
|   |       | 
|   |       ├── blockchain_usecase     # block chain usecase
|   |       |   ├── mod.rs
|   |       |   ├── get_blockchain_info.rs # Fetches & transforms chain state
|   |       |
|   |       ├── bexecute_raw_rpc_usecase.rs
|   |  
|   │
|   ├── infrastructure/             # 3. INFRASTRUCTURE LAYER (External Adapters & Drivers)
|   │   ├── mod.rs                  # Infrastructure module exports
|   │   ├── config.rs               # Environment & CLI flag configuration parser
|   │   │
|   │   └── bitcoin_rpc/            # JSON-RPC Integration for Bitcoin Core
|   │       ├── mod.rs
|   │       ├── client.rs           # Reqwest HTTP engine (handles Basic Auth & HTTP POSTs)
|   │       │
|   │       ├── dto/                # Infrastructure Serde DTOs (Matches raw RPC response JSON)
|   │       │   ├── mod.rs
|   │       │   ├── rpc_request.rs  # JSON-RPC 1.0 request structure
|   │       │   └── rpc_response.rs # Serde structs for Bitcoin Core JSON outputs
|   │       │
|   │       └── adapters/           # Concrete implementations of Domain Ports
|   │           ├── mod.rs
|   │           ├── blockchain_adapter.rs # Implements BlockchainPort using client.rs
|   │           ├── wallet_adapter.rs     # Implements WalletPort using client.rs
|   │           └── raw_rpc_adapter.rs    # Implements RawRpcPort using client.rs
|   │
|   └── presentation/               # 4. PRESENTATION LAYER (CLI Delivery Mechanism)
|       ├── mod.rs                  # Presentation module exports
|       ├── cli.rs                  # Clap command, argument & subcommand definitions
|       ├── formatters.rs           # Terminal output styling, table formatting & colors
|       │
|       └── handlers/               # Dispatches parsed CLI args to Container Use Cases
|           ├── mod.rs
|           ├── blockchain_handler.rs # Invokes GetBlockchainInfoUseCase
|           ├── wallet_handler.rs     # Invokes GetWalletInfo, GetBalance, GenerateAddress UCs
|           └── rpc_handler.rs        # Invokes ExecuteRawRpcUseCase
|
|
|
|
|
└── tests/                           # TEST DIRECTORY
    ├── common/
    │   └── mod.rs                   # Shared test helpers (e.g., node setup, test container)
    │
    ├── domain/                      # 1. Domain Layer Tests
    │   ├── value_objects/
    │   │   ├── address_test.rs
    │   │   └── balance_test.rs
    │   └── enums/
    │       └── address_type_test.rs
    │
    ├── application/                 # 2. Application Layer Tests
    │   └── use_cases/
    │       ├── get_balance_test.rs
    │       └── generate_address_test.rs
    │
    ├── infrastructure/              # 3. Infrastructure Layer Tests
    │   └── bitcoin_rpc/
    │       └── adapters/
    │           ├── blockchain_adapter_test.rs
    │           └── wallet_adapter_test.rs
    │
    └── presentation/                # 4. Presentation Layer Tests (CLI & Handlers)
        ├── cli_test.rs              # Tests Clap command parsing & flag defaults
        ├── formatters_test.rs       # Tests text & color formatting functions
        └── handlers/                # Integration tests for command execution handlers
            ├── blockchain_handler_test.rs
            ├── wallet_handler_test.rs
            └── rpc_handler_test.rs

```

# Getting Started

Follow these steps to set up your environment and run the application locally.

## 1. Prerequisites (Rust & Cargo)

Make sure you have the Rust toolchain installed.

- Download and run `rustup` from [https://rustup.rs/](https://rustup.rs/).
- Follow the on-screen prompts to complete default installation.
- Restart your PowerShell or Terminal window to reload path variables.
- Verify the installation:

```bash
rustc --version
cargo --version
```

## 2. Polar Setup

Polar allows you to spin up local Bitcoin Core and Lightning nodes in seconds.

- Download and launch [Polar](https://lightningpolar.com/).
-  * **Note:** dockerdesktop must be installed and running on your machine. Dont have docker yet? get it here, [Docker](https://www.docker.com/products/docker-desktop/) 
- Create a new Bitcoin Regtest network with at least one Bitcoin Core node (e.g., `backend1`).
- Start the network.
- (Optional) Create a default wallet inside your node using `bitcoin-cli`:

```bash
bitcoin-cli -regtest createwallet "my_wallet"
```

Expected Response:

```json
{
  "name": "my_wallet"
}
```

## 3. Environment Configuration

Create a `.env` file in the root directory by copying `.env.example`:

```bash
cp .env.example .env
```

Retrieve node credentials from Polar:

- Click on your running node (`backend1`).
- Select the **Connect** tab in the right sidebar pane.
- Locate the RPC Host, RPC User, and RPC Password.
- Update your `.env` file with these values:

```
BITCOIN_RPC_URL=http://127.0.0.1:18443
BITCOIN_RPC_USER=polaruser
BITCOIN_RPC_PASSWORD=polarpass
BTC_WALLET_NAME=my_wallet
```

## ⌖ Testing

Run the full test suite to verify that your domain rules, use case mocks, and Polar integrations are functional:

```bash
# Run all tests (unit and integration)
cargo test -- --nocapture

```

A passing test suite guarantees that both the mock architecture and live node connections are in a healthy state.

## 🤖 Usage & CLI Commands

Run the application using `cargo run -- <COMMAND>`.

### see application author's github username
```bash
cargo run -- auhtor
```
Example ouput:
```
╭──────────────────┬───────────╮
│ Core Contributor │ @fkwitty8 │
├──────────────────┼───────────┤
```

### 1. View Help

```bash
cargo run -- --help
```
Example output:
```
Bitcoin Core JSON-RPC CLI  v.0.0.1 by @fkwitty8

Usage:
  rfb_assesment.exe [OPTIONS] <COMMAND>

Commands:
  author           
  blockchain-info  Fetch blockchain status and metrics
  wallet-info      Fetch general information for the connected wallet
  balance          Query current balance in BTC
  new-address      Generate a new receiving address
  rpc              Execute an arbitrary raw JSON-RPC method against the Bitcoin node
  help             Print this message or the help of the given subcommand(s)

Options:
      --rpc-url <RPC_URL>    [env: BTC_RPC_URL=http://127.0.0.1:18443] [default: http://127.0.0.1:18443]
      --rpc-user <RPC_USER>  [env: BTC_RPC_USER]
      --rpc-pass <RPC_PASS>  [env: BTC_RPC_PASS]
      --wallet <WALLET>      [env: BTC_WALLET_NAME=my_wallet]
  -h, --help                 Print help
  -V, --version              Print version

```
### 2. Check Wallet Info

```bash
cargo run -- wallet-info
```

Example Output:

```
=== BITCOIN WALLET SUMMARY ===
╭─────────────────────┬───────────╮
│ Property            │ Value     │
├─────────────────────┼───────────┤
│ Wallet Name         │ my_wallet │
│ Balance             │ 0 BTC     │
│ Unconfirmed Balance │ 0 BTC     │
│ Transaction Count   │ 0         │
╰─────────────────────┴───────────╯
```

### 3. Get Wallet Balance

```bash
cargo run -- balance
```

Example Output:

```
=== BALANCE ===
╭────────────────┬───────╮
│ Property       │ Value │
├────────────────┼───────┤
│ Wallet Balance │ 0 BTC │
╰────────────────┴───────╯
```

### 4. Generate a New Receiving Address

```bash
cargo run -- new-address
```

Example output:

```
=== NEW ADDRESS ===
╭───────────────────┬──────────────────────────────────────────────╮
│ Property          │ Value                                        │
├───────────────────┼──────────────────────────────────────────────┤
│ Type              │ bech32                                       │
│ Generated Address │ bcrt1qx3apw3fvnz3hn9xnnwgzlrxcy7pycged5m0f3l │
╰───────────────────┴──────────────────────────────────────────────╯

```

### 5. Fetch Blockchain Summary

```bash
cargo run -- blockchain-info
```

Example output:
```
=== BITCOIN BLOCKCHAIN INFO ===
╭───────────────────────┬──────────────────────────────╮
│ Property              │ Value                        │
├───────────────────────┼──────────────────────────────┤
│ Chain                 │ regtest                      │
│ Blocks                │ 4                            │
│ Headers               │ 4                            │
│ Difficulty            │ 0.00000000046565423739069247 │
│ Verification Progress │ 22.57%                       │
╰───────────────────────┴──────────────────────────────╯
```

### 6. Execute Raw RPC Method
A.
```bash
cargo run -- rpc getblockcount
```
Example output:
```
╭────────────╮
│ RPC Output │
├────────────┤
│ 4          │
╰────────────╯
```
B.
```bash
cargo run -- rpc getblockhash 0
```
Example output:
```
╭──────────────────────────────────────────────────────────────────╮
│ RPC Output                                                       │
├──────────────────────────────────────────────────────────────────┤
│ 0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206 │
╰──────────────────────────────────────────────────────────────────╯
```
C.
```bash
cargo run -- rpc getblock 0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206
```

Example ouput:
```
╭───────────────┬──────────────────────────────────────────────────────────────────╮
│ Property      │ Value                                                            │
├───────────────┼──────────────────────────────────────────────────────────────────┤
│ bits          │ 207fffff                                                         │
│ chainwork     │ 0000000000000000000000000000000000000000000000000000000000000002 │
│ confirmations │ 5                                                                │
│ difficulty    │ 4.6565423739069247e-10                                           │
│ hash          │ 0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206 │
│ height        │ 0                                                                │
│ mediantime    │ 1296688602                                                       │
│ merkleroot    │ 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b │
│ nTx           │ 1                                                                │
│ nextblockhash │ 6693500991d79e48bd7c01b7a17d5e378aea6b291bd4606e0b74c8e3acac1d10 │
│ nonce         │ 2                                                                │
│ size          │ 285                                                              │
│ strippedsize  │ 285                                                              │
│ target        │ 7fffff0000000000000000000000000000000000000000000000000000000000 │
│ time          │ 1296688602                                                       │
│ tx            │ 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b │
│ version       │ 1                                                                │
│ versionHex    │ 00000001                                                         │
│ weight        │ 1140                                                             │
╰───────────────┴──────────────────────────────────────────────────────────────────╯

```