use std::{collections::HashMap, ops::Deref, sync::Arc};

use serde::Serialize;

use super::Monoid;

#[derive(Clone)]
struct Buckets<const N: usize>(Arc<[u64; N]>);

impl<const N: usize> Deref for Buckets<N> {
    type Target = [u64; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Serialize for Buckets<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = HashMap::new();
        for (k, v) in self.iter().enumerate() {
            map.insert(k, v);
        }
        map.serialize(serializer)
    }
}
/// N is the number of buckets exluding inf
/// buckets for values 0, 1, 2, .. N-1 are created
/// buckets[n] == x <=> there are x functions with n arguments
#[derive(Clone)]
pub struct Hist<const N: usize> {
    /// buckets length should never be changed
    buckets: Buckets<N>,
    /// number of observed values that are greater or equal to N
    inf: u64,
    /// observed values that dont fit into any of the buckets are put here
    inf_vals: Vec<usize>,
}

impl<const N: usize> Serialize for Hist<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.describe().serialize(serializer)
    }
}

#[derive(Serialize)]
pub struct HistSummary {
    pub sum: u64,
    pub count: u64,
    pub mean: f64,
    pub mode: Option<usize>,
}

impl<const N: usize> Monoid for Hist<N> {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl<const N: usize> Hist<N> {
    /// total number of observations
    pub fn count(&self) -> u64 {
        self.buckets.iter().sum::<u64>() + self.inf
    }
    /// total sum of the observed values
    pub fn sum(&self) -> u64 {
        self.buckets
            .into_iter()
            .enumerate()
            .map(|(val, count)| val as u64 * count)
            .sum::<u64>()
            + self.inf_vals.iter().sum::<usize>() as u64
    }

    pub fn mean(&self) -> f64 {
        self.sum() as f64 / self.count() as f64
    }

    /// None means that inf is the most frequent value,
    /// so the number of buckets should probably be increased
    pub fn mode(&self) -> Option<usize> {
        let (mode, &mode_observations) = self
            .buckets
            .iter()
            .enumerate()
            // not very correct as it returns the last value if
            // there are two maxes, but oh well
            .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
            .expect("empty histogram");
        if mode_observations <= self.inf {
            None
        } else {
            Some(mode)
        }
    }

    pub fn observe(&mut self, val: usize) {
        if val >= N {
            self.inf += 1;
            self.inf_vals.push(val);
        } else {
            Arc::make_mut(&mut self.buckets.0)[val] += 1;
        }
    }

    // TODO: macro
    pub fn describe(&self) -> HistSummary {
        HistSummary {
            sum: self.sum(),
            count: self.count(),
            mean: self.mean(),
            mode: self.mode(),
        }
    }
}

impl<const N: usize> Default for Hist<N> {
    fn default() -> Self {
        Self {
            buckets: Buckets(Arc::new([0; N])),
            inf: 0,
            inf_vals: Vec::with_capacity(0),
        }
    }
}

impl<const N: usize> std::ops::AddAssign for Hist<N> {
    fn add_assign(&mut self, mut rhs: Self) {
        // every bucket is guaranteed to exist as N is the same
        for (k, v) in rhs.buckets.into_iter().enumerate() {
            Arc::make_mut(&mut self.buckets.0)[k] += v;
        }
        self.inf += rhs.inf;
        self.inf_vals.append(&mut rhs.inf_vals);
    }
}

impl<const N: usize> std::ops::Add for Hist<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
