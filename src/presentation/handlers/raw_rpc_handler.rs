use crate::core::container::AppContainer;
use crate::core::exceptions::AppError;
use crate::presentation::formatters;
use serde_json::Value;

pub async fn handle_raw_rpc(
    container: &AppContainer,
    method: &str,
    raw_params: &[String],
) -> Result<(), AppError> {
    let parsed_params: Vec<Value> = raw_params
        .iter()
        .map(|p| {
            serde_json::from_str::<Value>(p).unwrap_or_else(|_| Value::String(p.clone()))
        })
        .collect();

    let result = container
        .execute_raw_rpc_uc
        .execute(method, parsed_params)
        .await?;

    formatters::print_raw_rpc(&result);
    Ok(())
}