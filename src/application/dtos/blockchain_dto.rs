use crate::domain::entities::blockchain::ChainState;

#[derive(Debug, Clone)]
pub struct BlockchainSummaryDto {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verification_progress: f64,
}

impl From<ChainState> for BlockchainSummaryDto {
    fn from(state: ChainState) -> Self {
        Self {
            chain: state.chain,
            blocks: state.blocks,
            headers: state.headers,
            difficulty: state.difficulty.value(),
            verification_progress: state.verification_progress,
        }
    }
}