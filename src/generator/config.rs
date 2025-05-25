#[derive(Debug, Clone)]
pub struct TransferGenConfig {
    pub min_amount: f64,
    pub max_amount: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub max_age_secs: u64,
}

impl Default for TransferGenConfig {
    fn default() -> Self {
        Self {
            min_amount: 1.0,
            max_amount: 1000.0,
            min_price: 0.1,
            max_price: 2.0,
            max_age_secs: 86_400 * 30,
        }
    }
}