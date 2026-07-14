use rfb_assesment::domain::enums::address_types_enums::AddressType;
use rfb_assesment::domain::ports::wallet_ports::WalletPort;
use rfb_assesment::infrastructure::bitcoin_rpc::adapters::wallet_adapters::BitcoinRpcWalletAdapter;
use rfb_assesment::infrastructure::bitcoin_rpc::client::BitcoinRpcClient;
use rfb_assesment::infrastructure::config::Config;
use std::sync::Arc;

fn setup_test_adapter() -> BitcoinRpcWalletAdapter {
    let config = Config {
        rpc_url: "http://127.0.0.1:18443".to_string(),
        rpc_user: "polaruser".to_string(),
        rpc_pass: "polarpass".to_string(),
        wallet: Some("default".to_string()),
    };

    let client = Arc::new(BitcoinRpcClient::new(&config));
    BitcoinRpcWalletAdapter::new(client)
}

#[tokio::test]
async fn test_get_balance_from_polar_node() {
    let adapter = setup_test_adapter();

    let result = adapter.get_balance().await;

    assert!(result.is_ok(), "Expected get_balance to succeed");
    assert!(result.unwrap().btc() >= 0.0, "Balance must be non-negative");
}

// #[tokio::test]
// async fn test_get_wallet_info_from_polar_node() {
//     let adapter = setup_test_adapter();

//     let result = adapter.get_wallet_info().await;

//     assert!(result.is_ok(), "Expected get_wallet_info to succeed");
//     let wallet = result.unwrap();
//     assert!(!wallet.name.is_empty(), "Wallet name should not be empty");
//     assert!(wallet.balance.btc() >= 0.0, "Wallet balance should be non-negative");
// }

#[tokio::test]
async fn test_get_wallet_info_from_polar_node() {
    let adapter = setup_test_adapter();

    let result = adapter.get_wallet_info().await;

    assert!(
        result.is_ok(),
        "\n\n❌ get_wallet_info failed with error: {:?}\n",
        result.err()
    );

    let wallet = result.unwrap();
    
    assert!(!wallet.name.is_empty(), "Wallet name should not be empty");
    assert_eq!(wallet.name, "default", "Expected wallet name to be 'default'");
}


#[tokio::test]
async fn test_generate_bech32_address() {
    let adapter = setup_test_adapter();

    let result = adapter.generate_address(AddressType::Bech32).await;

    assert!(result.is_ok(), "Expected bech32 address generation to succeed");
    let address = result.unwrap();
    
    // Regtest native segwit (bech32) addresses typically start with 'bcrt1'
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

    assert!(result.is_ok(), "Expected legacy address generation to succeed");
    let address = result.unwrap();
    
    // Regtest legacy addresses typically start with 'm' or 'n' (or '1' in mainnet)
    assert!(
        address.value().starts_with('m') || address.value().starts_with('n') || address.value().starts_with('1'),
        "Address should have a valid legacy prefix: {}",
        address.value()
    );
}