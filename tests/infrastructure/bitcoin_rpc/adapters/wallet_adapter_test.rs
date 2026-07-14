use clap::Parser;
use rfb_assesment::domain::enums::address_types_enums::AddressType;
use rfb_assesment::domain::ports::wallet_ports::WalletPort;
use rfb_assesment::infrastructure::bitcoin_rpc::adapters::wallet_adapters::BitcoinRpcWalletAdapter;
use rfb_assesment::infrastructure::bitcoin_rpc::client::BitcoinRpcClient;
use rfb_assesment::infrastructure::config::Config;
use std::sync::Arc;

#[derive(Parser)]
struct TestCli {
    #[command(flatten)]
    config: Config,
}

fn setup_test_adapter() -> BitcoinRpcWalletAdapter {
    // 1. Load variables from .env file into std::env
    dotenvy::dotenv().ok();

    // 2. Parse TestCli using an empty iterator so clap pulls directly from environment variables
    let cli = TestCli::parse_from(Vec::<String>::new());

    // 3. Construct the client and adapter using cli.config
    let client = Arc::new(BitcoinRpcClient::new(&cli.config));
    BitcoinRpcWalletAdapter::new(client)
}

#[tokio::test]
async fn test_get_balance_from_polar_node() {
    let adapter = setup_test_adapter();

    let result = adapter.get_balance().await;

    assert!(
        result.is_ok(),
        "\n\n ✕ Expected get_balance to succeed, got error: {:?}\n",
        result.err()
    );
    assert!(
        result.unwrap().btc() >= 0.0,
        "Balance must be non-negative"
    );
}

#[tokio::test]
async fn test_get_wallet_info_from_polar_node() {
    let adapter = setup_test_adapter();

    let result = adapter.get_wallet_info().await;

    assert!(
        result.is_ok(),
        "\n\n ✕ Expected get_wallet_info to succeed, got error: {:?}\n",
        result.err()
    );

    let wallet = result.unwrap();

    assert!(!wallet.name.is_empty(), "Wallet name should not be empty");
    assert_eq!(
        wallet.name, "default",
        "Expected wallet name to match configured wallet"
    );
    assert!(
        wallet.balance.btc() >= 0.0,
        "Wallet balance should be non-negative"
    );
}

#[tokio::test]
async fn test_generate_bech32_address() {
    let adapter = setup_test_adapter();

    let result = adapter.generate_address(AddressType::Bech32).await;

    assert!(
        result.is_ok(),
        "\n\n ✕ Expected bech32 address generation to succeed, got error: {:?}\n",
        result.err()
    );
    let address = result.unwrap();

    // Regtest native segwit (bech32) addresses typically start with 'bcrt1' or 'bc1'
    assert!(
        address.value().starts_with("bcrt1") || address.value().starts_with("bc1"),
        "Address should have a valid Bech32 prefix: {}",
        address.value()
    );
}

#[tokio::test]
async fn test_generate_legacy_address() {
    let adapter = setup_test_adapter();

    let result = adapter.generate_address(AddressType::Legacy).await;

    assert!(
        result.is_ok(),
        "\n\n ✕ Expected legacy address generation to succeed, got error: {:?}\n",
        result.err()
    );
    let address = result.unwrap();

    // Regtest legacy addresses typically start with 'm', 'n', or '1'
    assert!(
        address.value().starts_with('m')
            || address.value().starts_with('n')
            || address.value().starts_with('1'),
        "Address should have a valid legacy prefix: {}",
        address.value()
    );
}