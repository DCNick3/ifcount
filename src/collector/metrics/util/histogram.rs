use rustc_hash::FxHashMap;

use serde::Serialize;

use super::Monoid;

/// N is the number of buckets exluding inf
/// buckets for values 0, 1, 2, .. N-1 are created
/// buckets[n] == x <=> there are x functions with n arguments
#[derive(Clone)]
pub struct Hist {
    /// buckets length should never be changed
    buckets: FxHashMap<usize, u32>,
}

impl Serialize for Hist {
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
    pub avg: f64,
    pub mode: Option<usize>,
}

impl Monoid for Hist {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Hist {
    /// total number of observations
    pub fn count(&self) -> u64 {
        self.buckets.values().map(|&v| v as u64).sum::<u64>()
    }
    /// total sum of the observed values
    pub fn sum(&self) -> u64 {
        self.buckets
            .iter()
            .map(|(&val, &count)| val as u64 * count as u64)
            .sum::<u64>()
    }

    /// average value of the observed values
    pub fn average(&self) -> f64 {
        self.sum() as f64 / self.count() as f64
    }

    /// None means that inf is the most frequent value,
    /// so the number of buckets should probably be increased
    pub fn mode(&self) -> Option<usize> {
        self.buckets
            .iter()
            // not very correct as it returns the last value if
            // there are two maxes, but oh well
            .max_by(|(_, &count1), (_, &count2)| count1.cmp(&count2))
            .map(|(&mode, _)| mode)
    }

    pub fn observe(&mut self, val: usize) {
        *self.buckets.entry(val).or_insert(0) += 1;
    }

    pub fn describe(&self) -> HistSummary {
        HistSummary {
            sum: self.sum(),
            avg: self.average(),
            mode: self.mode(),
        }
    }
}

impl Default for Hist {
    fn default() -> Self {
        Self {
            buckets: FxHashMap::default(),
        }
    }
}

impl std::ops::AddAssign for Hist {
    fn add_assign(&mut self, rhs: Self) {
        // every bucket is guaranteed to exist as N is the same
        for (k, v) in rhs.buckets.into_iter() {
            *self.buckets.entry(k).or_insert(0) += v;
        }
    }
}

impl std::ops::Add for Hist {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
