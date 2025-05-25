use crate::model::{Transfer, UserStats};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests;

pub trait StatsCalculator {
    fn calculate_user_stats(&self, transfers: &[Transfer]) -> Vec<UserStats>;
}


#[derive(Default)]
pub struct MockCalculator {}

impl StatsCalculator for MockCalculator {
    fn calculate_user_stats(&self, transfers: &[Transfer]) -> Vec<UserStats> {
        let mut balances: HashMap<String, f64> = HashMap::new();
        let mut max_balances: HashMap<String, f64> = HashMap::new();
        let mut buy_prices: HashMap<String, (f64, f64)> = HashMap::new();
        let mut sell_prices: HashMap<String, (f64, f64)> = HashMap::new();
        for t in transfers {
            *balances.entry(t.from.clone()).or_default() -= t.amount * t.usd_price;
            *balances.entry(t.to.clone()).or_default() += t.amount * t.usd_price;

            let to_balance = balances.get(&t.to).copied().unwrap_or(0.0);
            let from_balance = balances.get(&t.from).copied().unwrap_or(0.0);
            max_balances.entry(t.to.clone()).and_modify(|b| *b = b.max(to_balance)).or_insert(to_balance);
            max_balances.entry(t.from.clone()).and_modify(|b| *b = b.max(from_balance)).or_insert(from_balance);

            buy_prices.entry(t.to.clone()).and_modify(|(p, a)| {
                *p += t.usd_price * t.amount;
                *a += t.amount
            }).or_insert((t.usd_price * t.amount, t.amount));
            sell_prices.entry(t.from.clone()).and_modify(|(p, a)| {
                *p += t.usd_price * t.amount;
                *a += t.amount
            }).or_insert((t.usd_price * t.amount, t.amount));
        }

        let mut used_addresses = HashSet::new();

        buy_prices.keys().into_iter().chain(sell_prices.keys().into_iter())
            .filter_map(|addr| {
                if used_addresses.contains(addr) {
                    return None;
                }
                used_addresses.insert(addr.clone());
                let buys = buy_prices.get(addr).copied().unwrap_or_default();
                let sells = sell_prices.get(addr).copied().unwrap_or_default();
                let mut total_volume = buys.1 + sells.1;
                let avg = |p, a| {
                    if a > 0.0 {
                        p / a
                    } else {
                        0.0
                    }
                };

                Some(UserStats {
                    address: addr.clone(),
                    total_volume,
                    avg_buy_price: avg(buys.0, buys.1),
                    avg_sell_price: avg(sells.0, sells.1),
                    max_balance: *max_balances.get(addr).unwrap_or(&0.0),
                })
            })
            .collect()
    }
}

