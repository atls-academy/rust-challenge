use chrono::{DateTime, Utc};
use crate::model::Transfer;

pub mod mock;

#[derive(Default)]
pub struct Query {
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub min_amount: Option<f64>,
    pub addresses: Option<Vec<String>>,
}

pub trait Storage {
    fn save_transfers(&mut self, transfers: Vec<Transfer>) -> anyhow::Result<()>;
    fn get_transfers(&self, param: Query) -> anyhow::Result<Vec<Transfer>>;
}


#[cfg(test)]
mod tests;