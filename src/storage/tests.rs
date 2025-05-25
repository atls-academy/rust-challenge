use std::ops::{Add, Sub};
use chrono::{DateTime, Duration, Utc};
use crate::model::Transfer;
use crate::storage::mock::MockStorage;
use crate::storage::{Query, Storage};

#[test]
fn simple() {
    let mut s = MockStorage::default();
    let mut q = Query::default();
    let now = Utc::now();
    q.from_date = Some(now.sub(Duration::seconds(2)));
    println!("{}", DateTime::from_timestamp(now.sub(Duration::seconds(3)).timestamp() as u64 as i64, 0).unwrap());
    s.save_transfers(vec![Transfer {
        ts: now.sub(Duration::seconds(3)).timestamp() as u64,
        from: "a".to_string(),
        to: "b".to_string(),
        amount: 5.0,
        usd_price: 1.1,
    }, Transfer {
        ts: now.timestamp() as u64,
        from: "b".to_string(),
        to: "c".to_string(),
        amount: 2.0,
        usd_price: 2.0,
    }, Transfer {
        ts: now.add(Duration::seconds(2)).timestamp() as u64,
        from: "c".to_string(),
        to: "d".to_string(),
        amount: 1.0,
        usd_price: 3.5,
    },
    ]).unwrap();

    let res = s.get_transfers(q).unwrap();
    assert_eq!(res[0].from, "b");
    assert_eq!(res[0].from, "c");
}

#[test]
fn error() {
    let mut s = MockStorage::default();
    s.save_transfers(vec![Transfer {
        ts: 0,
        from: "a".to_string(),
        to: "b".to_string(),
        amount: 5.0,
        usd_price: 1.1,
    }, Transfer {
        ts: 1,
        from: "b".to_string(),
        to: "c".to_string(),
        amount: 2.0,
        usd_price: 2.0,
    }, Transfer {
        ts: 2,
        from: "c".to_string(),
        to: "d".to_string(),
        amount: 1.0,
        usd_price: 3.5,
    },
    ]).unwrap();
    let mut q = Query::default();
    let now = Utc::now();
    q.from_date = Some(now.sub(Duration::seconds(2)));
    q.to_date = Some(now.sub(Duration::seconds(200)));
    let err = s.get_transfers(q);
    assert!(err.is_err());
}