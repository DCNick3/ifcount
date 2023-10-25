use rustc_hash::FxHashMap;

use serde::Serialize;

use super::{super::Monoid, Observer};

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

impl Observer for Hist {
    fn observe(&mut self, value: usize) {
        self.observe(value);
    }

    fn count(&self) -> usize {
        self.count() as usize
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
        let mut buckets: Vec<_> = self.buckets.iter().collect();
        // so that mode is determenistic on equal values
        buckets.sort_by(|(val1, _), (val2, _)| val1.cmp(val2));
        buckets
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

    pub fn into_values(self) -> Vec<usize> {
        let mut out = Vec::with_capacity(self.count() as usize);
        let mut pairs = self.buckets.into_iter().collect::<Vec<_>>();
        pairs.sort();
        for (val, count) in pairs {
            for _ in 0..count {
                out.push(val);
            }
        }
        out
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

#[cfg(test)]
mod tests {
    use crate::collector::metrics::util::Monoid;

    #[test]
    fn test_hist() {
        let mut hist = super::Hist::default();
        hist.observe(1);

        hist.observe(2);
        hist.observe(2);

        hist.observe(3);
        hist.observe(3);
        hist.observe(3);

        hist.observe(4);
        hist.observe(4);
        hist.observe(4);
        hist.observe(4);

        hist.observe(5);
        hist.observe(5);
        hist.observe(5);
        hist.observe(5);
        hist.observe(5);

        assert_eq!(hist.count(), 15);
        assert_eq!(hist.sum(), 55);
        assert_eq!(hist.average(), 55.0 / 15.0);
        assert_eq!(hist.mode(), Some(5));
        assert_eq!(
            hist.into_values(),
            vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]
        );
    }

    #[test]
    fn merge_hist() {
        let mut hist1 = super::Hist::default();
        hist1.observe(1);
        hist1.observe(2);
        hist1.observe(3);
        hist1.observe(4);

        let mut hist2 = super::Hist::default();
        hist2.observe(1);
        hist2.observe(5);
        hist2.observe(6);
        hist2.observe(1);

        hist1 += hist2;

        assert_eq!(hist1.count(), 8);
        assert_eq!(hist1.sum(), 23);
        assert_eq!(hist1.average(), 23.0 / 8.0);
        assert_eq!(hist1.mode(), Some(1));
        assert_eq!(hist1.into_values(), vec![1, 1, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn monoid() {
        let mut hist1 = super::Hist::init();
        hist1.observe(1);
        hist1.observe(2);
        hist1.observe(3);
        hist1.observe(4);

        let mut hist2 = super::Hist::init();
        hist2.observe(1);
        hist2.observe(5);
        hist2.observe(6);
        hist2.observe(1);

        let hist = super::Hist::unite(hist1, hist2);

        assert_eq!(hist.count(), 8);
        assert_eq!(hist.sum(), 23);
        assert_eq!(hist.average(), 23.0 / 8.0);
        assert_eq!(hist.mode(), Some(1));
        assert_eq!(hist.into_values(), vec![1, 1, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn determenistic_mode() {
        let mut hist1 = super::Hist::init();
        hist1.observe(1);
        hist1.observe(1);
        hist1.observe(2);
        hist1.observe(2);
        assert_eq!(hist1.mode(), Some(2));

        let mut hist2 = super::Hist::init();
        hist2.observe(0);
        hist2.observe(1);
        hist2.observe(1);
        hist2.observe(1);
        hist2.observe(5);
        hist2.observe(5);
        hist2.observe(5);
        hist2.observe(6);
        assert_eq!(hist2.mode(), Some(5));
    }
}
