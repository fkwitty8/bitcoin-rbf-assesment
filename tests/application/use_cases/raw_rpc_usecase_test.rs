// use std::sync::Arc;
// use serde_json::json;

// use rfb_assesment::application::use_cases::blockchain_usecases::get_blockchain_info_usecase::GetBlockchainInfoUseCase;
// use rfb_assesment::application::use_cases::execute_raw_rpc_usecase::ExecuteRawRpcUseCase;




// use rfb_assesment::domain::exceptions::DomainError;



// use rfb_assesment::domain::ports::blockchain_ports::MockBlockchainPort;
// use rfb_assesment::domain::ports::raw_rpc_ports::MockRawRpcPort;
// use rfb_assesment::domain::ports::wallet_ports::MockWalletPort;




// use mockall::mock;



// mock! {
//     pub RawRpcPort {}

//     #[async_trait]
//     impl RawRpcPort for RawRpcPort {
//         async fn execute(&self, method: &str, params: Vec<Value>) -> Result<Value, DomainError>;
//     }
// }

// #[tokio::test]
// async fn test_execute_raw_rpc_success() {
//     let mut mock_port = MockRawRpcPort::new();

//     mock_port
//         .expect_execute()
//         .withf(|method, params| method == "getblockcount" && params.is_empty())
//         .times(1)
//         .returning(|_, _| Ok(json!(100)));

//     let use_case = ExecuteRawRpcUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute("getblockcount", vec![]).await;

//     assert!(result.is_ok());
//     assert_eq!(result.unwrap(), json!(100));
// }

// #[tokio::test]
// async fn test_execute_raw_rpc_failure() {
//     let mut mock_port = MockRawRpcPort::new();

//     mock_port
//         .expect_execute()
//         .times(1)
//         .returning(|_, _| Err(DomainError::Infrastructure("RPC call failed".to_string())));

//     let use_case = ExecuteRawRpcUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute("invalid_method", vec![]).await;

//     assert!(result.is_err());
// }