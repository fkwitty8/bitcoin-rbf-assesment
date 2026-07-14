use rfb_assesment::core::container::AppContainer;
use rfb_assesment::infrastructure::config::Config;
use rfb_assesment::presentation::handlers::wallet_handler;

#[tokio::test]
async fn test_handle_balance_execution() {
    let config = Config {
        rpc_url: "http://127.0.0.1:18443".to_string(),
        rpc_user: "polaruser".to_string(),
        rpc_pass: "polarpass".to_string(),
        wallet: None,
    };

    let container = AppContainer::new(&config);
    let result = wallet_handler::handle_balance(&container).await;

    assert!(result.is_ok());
}