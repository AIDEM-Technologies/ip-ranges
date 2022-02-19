use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::*;

mod tests;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IpRanges {
    firsts: Vec<u32>,
    lasts: Vec<u32>,
}

impl IpRanges {
    pub fn new_from_cidr(cidr: Vec<cidr_utils::cidr::Ipv4Cidr>) -> Self {
        let intervals = cidr.iter().map(|cidr| rust_lapper::Interval {
            start: cidr.first(),
            stop: cidr.last(),
            val: 0,
        }).collect();
        let mut lapper = rust_lapper::Lapper::new(intervals);
        lapper.merge_overlaps();
        let intervals = lapper.iter().collect_vec();
        let mut firsts = intervals.iter().map(|i| i.start).collect_vec();
        let mut lasts = intervals.iter().map(|i| i.stop).collect_vec();
        firsts.sort();
        lasts.sort();
        Self::new(firsts, lasts).expect("This can't fail because we are creating directly from CIDRs")
    }
    pub fn new(mut firsts: Vec<u32>, mut lasts: Vec<u32>) -> Result<Self> {
        firsts.sort();
        lasts.sort();
        firsts.dedup();
        lasts.dedup();
        if firsts.len() != lasts.len() {
            return Err(anyhow!("Input vectors do not have the same length"));
        }
        for (start, end) in firsts.iter().zip(lasts.iter()) {
            if end < start {
                return Err(anyhow!("Input vectors do not represent a valid collection of ranges"));
            }
        }
        Ok(Self {
            firsts,
            lasts,
        })
    }
    pub fn contains(&self, value: u32) -> bool {
        if value < *self.firsts.first().unwrap() {
            return false;
        }
        match self.firsts.binary_search(&value) {
            Ok(_) => return true,
            Err(index) => {
                let first_value = self.firsts[index - 1];
                let last_value = self.lasts[index - 1];
                trace!("{}<={}<={} is {}", first_value, value, last_value, first_value <= value && value <= last_value);
                if first_value <= value && value <= last_value {
                    return true;
                }
                false
            }
        }
    }
    pub fn contains_ip(&self, ip: std::net::Ipv4Addr) -> bool {
        let value = ip.octets().into_iter().map(|t| t as u32).reduce(|a, b| a * 256 + b).expect("This should never happen");
        self.contains(value)
    }
}