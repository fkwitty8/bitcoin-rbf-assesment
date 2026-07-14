#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Difficulty(f64);

impl Difficulty {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}