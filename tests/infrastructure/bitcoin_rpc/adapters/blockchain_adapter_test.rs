use rfb_assesment::domain::ports::blockchain_ports::BlockchainPort;
use rfb_assesment::infrastructure::bitcoin_rpc::adapters::blockchain_adapters::BitcoinRpcBlockchainAdapter;
use rfb_assesment::infrastructure::bitcoin_rpc::client::BitcoinRpcClient;
use rfb_assesment::infrastructure::config::Config;
use std::sync::Arc;

fn setup_blockchain_adapter() -> BitcoinRpcBlockchainAdapter {
    let config = Config {
        rpc_url: "http://127.0.0.1:18443".to_string(),
        rpc_user: "polaruser".to_string(),
        rpc_pass: "polarpass".to_string(),
        wallet: None,
    };

    let client = Arc::new(BitcoinRpcClient::new(&config));
    BitcoinRpcBlockchainAdapter::new(client)
}

#[tokio::test]
async fn test_get_chain_state_from_polar_node() {
    let adapter = setup_blockchain_adapter();

    let result = adapter.get_chain_state().await;
    

    assert!(result.is_ok(), "Expected get_chain_state to succeed");
    let chain_state = result.unwrap();

    // Verify chain network (Polar default is "regtest")
    assert_eq!(chain_state.chain, "regtest", "Expected chain network to be regtest");
    assert!(chain_state.verification_progress >= 0.0, "Verification progress should be valid");
}