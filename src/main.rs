mod model;
mod generator;
mod pipeline;
mod storage;

use std::ops::Sub;
use anyhow::Context;
use chrono::{DateTime, Duration, Utc};
use crate::generator::{DefaultTransferGenerator, TransferGenerator};
use crate::model::Transfer;
use crate::pipeline::{MockCalculator, StatsCalculator};
use crate::storage::{Query, Storage};
use crate::storage::mock::MockStorage;

fn main() {
    sync_main().context("Main execution has been aborted: ").unwrap();
}

fn sync_main()->anyhow::Result<()>{
    let mut storage = MockStorage::default();
    let calculator = MockCalculator::default();
    let transfers = DefaultTransferGenerator::default().generate(1000000);

    storage.save_transfers(transfers).context("Storage.save_transfers error: ")?;

    let all_transfers = storage.get_transfers(Query::default()).context("storage.get_transfers error")?;
    let default_stats = calculator.calculate_user_stats(&all_transfers);
    for stat in default_stats.iter().take(10) {
        println!("{:?}", stat);
    }

    let mut query_half_month = Query::default();
    query_half_month.from_date = Some(Utc::now().sub(Duration::days(15)));
    let half_year = storage.get_transfers(query_half_month)?;
    println!("in the previous month there were {} transfers", all_transfers.len());
    println!("in the previous half of month there were {} transfers", half_year.len());
    let half_year_stats = calculator.calculate_user_stats(&half_year);
    println!("half year stats");
    for stat in half_year_stats.iter().take(10) {
        println!("{:?}", stat);
    }
    Ok(())
}
