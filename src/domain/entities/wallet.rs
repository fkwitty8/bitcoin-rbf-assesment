use crate::domain::value_objects::balance::Balance;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub name: String,
    pub balance: Balance,
    pub unconfirmed_balance: Balance,
    pub tx_count: u64,
}

impl Wallet {
    pub fn new(
        name: String,
        balance: Balance,
        unconfirmed_balance: Balance,
        tx_count: u64,
    ) -> Self {
        Self {
            name,
            balance,
            unconfirmed_balance,
            tx_count,
        }
    }
}