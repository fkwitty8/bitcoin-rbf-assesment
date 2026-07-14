use crate::core::container::AppContainer;
use crate::core::exceptions::AppError;
use crate::domain::enums::address_types_enums::AddressType;
use crate::presentation::formatters;
use std::str::FromStr;


pub async fn handle_wallet_info(container: &AppContainer) -> Result<(), AppError> {
    let dto = container.get_wallet_info_uc.execute().await?;
    formatters::print_wallet_info(&dto);
    Ok(())
}

pub async fn handle_balance(container: &AppContainer) -> Result<(), AppError> {
    let balance = container.get_balance_uc.execute().await?;
    formatters::print_balance(balance);
    Ok(())
}

pub async fn handle_new_address(
    container: &AppContainer,
    raw_addr_type: &str,
) -> Result<(), AppError> {
    let addr_type = AddressType::from_str(raw_addr_type)?;
    let address = container.generate_address_uc.execute(addr_type).await?;
    formatters::print_address(&address, addr_type.as_str());
    Ok(())
}