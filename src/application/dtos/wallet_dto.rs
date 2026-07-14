use crate::domain::entities::wallet::Wallet;

#[derive(Debug, Clone)]
pub struct WalletSummaryDto {
    pub name: String,
    pub balance_btc: f64,
    pub unconfirmed_balance_btc: f64,
    pub tx_count: u64,
}

impl From<Wallet> for WalletSummaryDto {
    fn from(wallet: Wallet) -> Self {
        Self {
            name: wallet.name,
            balance_btc: wallet.balance.btc(),
            unconfirmed_balance_btc: wallet.unconfirmed_balance.btc(),
            tx_count: wallet.tx_count,
        }
    }
}