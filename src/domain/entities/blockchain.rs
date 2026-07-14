use crate::domain::value_objects::difficulty::Difficulty;

#[derive(Debug, Clone)]
pub struct ChainState {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: Difficulty,
    pub verification_progress: f64,
}

impl ChainState {
    pub fn new(
        chain: String,
        blocks: u64,
        headers: u64,
        difficulty: f64,
        verification_progress: f64,
    ) -> Self {
        Self {
            chain,
            blocks,
            headers,
            difficulty: Difficulty::new(difficulty),
            verification_progress,
        }
    }
}