use crate::core::container::AppContainer;
use crate::core::exceptions::AppError;
use crate::presentation::formatters;

pub async fn handle_blockchain_info(container: &AppContainer) -> Result<(), AppError> {
    let dto = container.get_blockchain_info_uc.execute().await?;
    formatters::print_blockchain_info(&dto);
    Ok(())
}