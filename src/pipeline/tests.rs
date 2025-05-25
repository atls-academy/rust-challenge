use crate::model::{Transfer, UserStats};
use crate::pipeline::{MockCalculator, StatsCalculator};

#[test]
fn simple() {
    let transfers = vec![Transfer {
        ts: 1,
        from: "a".to_string(),
        to: "b".to_string(),
        amount: 5.0,
        usd_price: 1.1,
    }, Transfer {
        ts: 2,
        from: "b".to_string(),
        to: "c".to_string(),
        amount: 2.0,
        usd_price: 2.0,
    }, Transfer {
        ts: 3,
        from: "b".to_string(),
        to: "c".to_string(),
        amount: 1.0,
        usd_price: 3.5,
    },
    ];
    let mut res = MockCalculator::default().calculate_user_stats(&transfers);
    res.sort_by(|x, x1| {
        x.address.cmp(&x1.address)
    });

    assert_eq!(res[0], UserStats {
        address: "a".to_string(),
        total_volume: 5.0,
        avg_buy_price: 0.0,
        avg_sell_price: 1.1,
        max_balance: -5.5,
    });
    assert_eq!(res[1], UserStats {
        address: "b".to_string(),
        total_volume: 8.0,
        avg_buy_price: 1.1,
        avg_sell_price: 2.5,
        max_balance: 5.5,
    });
    assert_eq!(res[2], UserStats {
        address: "c".to_string(),
        total_volume: 3.0,
        avg_buy_price: 2.5,
        avg_sell_price: 0.0,
        max_balance: 7.5,
    });
    println!("{:?}", res);
}