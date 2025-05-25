use std::collections::{BTreeMap, HashMap, HashSet};
use std::collections::btree_map::IntoIter;
use anyhow::bail;
use chrono::{DateTime, Utc};
use crate::model::Transfer;
use crate::storage::{Query, Storage};



// Mock storage
// Effective search by time O(logn)
// Ineffective search by amount and addresses O(n)
#[derive(Default)]
pub struct MockStorage {
    inner_buf: BTreeMap<DateTime<Utc>, Vec<Transfer>>,
}


impl Storage for MockStorage {
    fn save_transfers(&mut self, transfers: Vec<Transfer>) -> anyhow::Result<()> {
        for t in transfers {
            let utc_time = DateTime::from_timestamp(t.ts as i64, 0);
            match utc_time {
                None => {
                    bail!("incorrect time")
                }
                Some(v) => {
                    self.inner_buf.entry(v).or_default().push(t);
                }
            }
        }
        Ok(())
    }

    fn get_transfers(&self, param: Query) -> anyhow::Result<Vec<Transfer>> {
        let iter: &mut dyn Iterator<Item=(&DateTime<Utc>, &Vec<Transfer>)> = match param {
            Query { from_date: Some(from), to_date: Some(to), .. } => {
                if from >= to {
                    bail!("from date {} may not precede to date {}",from, to )
                }
                let mut  r = self.inner_buf.range(from..to);
                &mut r.into_iter() as &mut dyn Iterator<Item=(&DateTime<Utc>, &Vec<Transfer>)>
            },
            Query { from_date: Some(from), .. } => {
                let mut  r = self.inner_buf.range(from..);
                &mut r.into_iter() as &mut dyn Iterator<Item=(&DateTime<Utc>, &Vec<Transfer>)>
            },
            Query { to_date: Some(to), .. } => {
                let mut  r = self.inner_buf.range(..to);
                &mut r.into_iter() as &mut dyn Iterator<Item=(&DateTime<Utc>, &Vec<Transfer>)>
            },
            _ => {
                &mut self.inner_buf.iter() as &mut dyn Iterator<Item=(&DateTime<Utc>, &Vec<Transfer>)>
            }
        };
        let mut addresses = HashSet::new();
        let have_addresses = param.addresses.is_some();
        if let Some(add) = param.addresses {
            addresses = add.into_iter().collect();
        }
        let out = iter.map(|(key, val)| {
            let o: Vec<Transfer> = val.into_iter().filter(|x| {
                if let Some(min_amount) = param.min_amount {
                    if x.amount < min_amount {
                        return false;
                    }
                }
                if have_addresses && !(addresses.contains(&x.to) || addresses.contains(&x.from)) {
                    return false;
                }
                return true;
            }).map(|x1| {x1.clone()}).collect();
            o
        }).flatten().collect();
        Ok(out)
    }
}