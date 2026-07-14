# Phase 1: Project Setup & Core Configuration
Start by getting the project building and reading Polar credentials.

* **Cargo.toml**: Add dependencies (`clap`, `tokio`, `reqwest`, `serde`, `serde_json`, `thiserror`, `colored`, `dotenvy`, `async-trait`).
* **.env**: Store Polar credentials:
  ```env
  BTC_RPC_URL=[http://127.0.0.1:18443](http://127.0.0.1:18443)
  BTC_RPC_USER=polaruser
  BTC_RPC_PASS=polarpass
  
* **src/infrastructure/config.rs:** Create `Config` struct to load `.env` variables and CLI flag overrides.
* **src/core/error.rs:** Set up top-level `AppError` enum using `thiserror`.

---

# Phase 2: The Domain Layer (Core Rules)
Define domain types and interfaces. This layer has zero external crate dependencies.

* **domain/errors.rs:** Define `DomainError`.
* **domain/enums/address_type.rs:** Define `AddressType` (`Bech32`, `Legacy`, `P2shSegwit`).
* **domain/value_objects/:**
  * `address.rs` (Bitcoin address string wrapper)
  * `balance.rs` (BTC balance type)
* **domain/entities/:**
  * `blockchain.rs` (`ChainState` entity: blocks, headers, difficulty)
  * `wallet.rs` (`Wallet` entity: name, balance, tx count)
* **domain/ports/:** Define the async traits:
  * `blockchain_port.rs`
  * `wallet_port.rs`
  * `raw_rpc_port.rs`

---

# Phase 3: Infrastructure Layer (Bitcoin Core JSON-RPC Engine)
wire up the HTTP client that actually talks to Polar.

* **infrastructure/bitcoin_rpc/dto/:**
  * `rpc_request.rs`: Struct for `{"jsonrpc": "1.0", "id": "...", "method": "...", "params": [...]}`.
  * `rpc_response.rs`: Structs for `getblockchaininfo` and `getwalletinfo` JSON parsing.
* **infrastructure/bitcoin_rpc/client.rs:** Build the `reqwest` HTTP client with Basic Auth (`polaruser` / `polarpass`).
* **infrastructure/bitcoin_rpc/adapters/:**
  * Implement `BlockchainPort` in `blockchain_adapter.rs`.
  * Implement `WalletPort` in `wallet_adapter.rs`.
  * Implement `RawRpcPort` in `raw_rpc_adapter.rs`.

---

# Phase 4: Application Layer (Use Cases)
Bridge the presentation and infrastructure by writing application orchestrators.

* **application/dtos/:** Define output structures for presentation formatting.
* **application/use_cases/:**
  * Write `get_blockchain_info.rs`
  * Write `get_wallet_info.rs`, `get_balance.rs`, `generate_address.rs`
  * Write `execute_raw_rpc.rs`

---

# Phase 5: Dependency Injection & Presentation (CLI)
Wire everything together and connect terminal inputs.

* **core/container.rs:** Build `AppContainer` to instantiate `client.rs`, adapters, and use cases.
* **presentation/cli.rs:** Define subcommands and flags using `clap`.
* **presentation/formatters.rs:** Add terminal output colors with `colored`.
* **presentation/handlers/:** Wire CLI arguments to `AppContainer` use cases.
* **src/main.rs:** Boot the app, load `.env`, instantiate `AppContainer`, and execute the CLI handler.

---
