use std::{collections::HashMap, ops::Deref, sync::Arc};

use serde::Serialize;
use syn::visit::{self, Visit};

use super::prelude::*;

#[derive(Clone)]
struct Buckets<const N: usize>(Arc<[u64; N]>);

impl<const N: usize> Deref for Buckets<N> {
    type Target = [u64; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl<const N: usize> DerefMut for Buckets<N> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

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
/// buckets[n] == x <=> there are x functions with n arguments
#[derive(Clone, Serialize)]
pub struct FnArgsHist<const N: usize> {
    buckets: Buckets<N>, // buckets length should never be changed
    inf: u64,
    inf_vals: Vec<usize>, // put observed values that dont fit into buckets here
}

impl<const N: usize> FnArgsHist<N> {
    /// total number of observations
    fn count(&self) -> u64 {
        self.buckets.iter().sum::<u64>() + self.inf
    }
    /// total sum of the observed values
    fn sum(&self) -> u64 {
        self.buckets
            .into_iter()
            .enumerate()
            .map(|(val, count)| val as u64 * count)
            .sum::<u64>()
            + self.inf_vals.iter().sum::<usize>() as u64
    }
}

impl<const N: usize> Default for FnArgsHist<N> {
    fn default() -> Self {
        Self {
            buckets: Buckets(Arc::new([0; N])),
            inf: 0,
            inf_vals: Vec::with_capacity(0),
        }
    }
}

impl<const N: usize> std::ops::AddAssign for FnArgsHist<N> {
    fn add_assign(&mut self, mut rhs: Self) {
        // every bucket is guaranteed to exist as N is the same
        for (k, v) in rhs.buckets.into_iter().enumerate() {
            Arc::make_mut(&mut self.buckets.0)[k] += v;
        }
        self.inf += rhs.inf;
        self.inf_vals.append(&mut rhs.inf_vals);
    }
}

impl<const N: usize> std::ops::Add for FnArgsHist<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize> Visit<'_> for FnArgsHist<N> {
    fn visit_signature(&mut self, i: &'_ syn::Signature) {
        let arg_count = i.inputs.len();
        if arg_count >= self.buckets.len() {
            self.inf += 1;
            self.inf_vals.push(arg_count);
        } else {
            Arc::make_mut(&mut self.buckets.0)[arg_count] += 1;
        }
        visit::visit_signature(self, i);
    }
}

#[derive(Default)]
pub struct FnArgsAvg(FnArgsHist<16>);

impl Visit<'_> for FnArgsAvg {
    fn visit_signature(&mut self, i: &'_ syn::Signature) {
        self.0.visit_signature(i);
    }
}

impl Visitor for FnArgsAvg {
    fn visitor() -> super::prelude::MetricCollectorBox {
        util::VisitorCollector::new(
            "avg_fn_arg_count",
            FnArgsAvg::default(),
            |v| v,
            |v| {
                let total_hist: FnArgsHist<16> = v
                    .into_iter()
                    .map(|FnArgsAvg(hist)| hist.to_owned())
                    .fold(FnArgsHist::default(), |x, y| x + y);
                total_hist.sum() as f64 / total_hist.count() as f64
            },
        )
        .make_box()
    }
}

impl<const N: usize> Visitor for FnArgsHist<N> {
    fn visitor() -> MetricCollectorBox {
        util::VisitorCollector::new(
            "fn_arg_histogram",
            FnArgsHist::default(),
            |v| v,
            |v: &[FnArgsHist<N>]| {
                v.into_iter()
                    .map(|hist| hist.to_owned())
                    .fold(FnArgsHist::default(), |acc, x| acc + x)
            },
        )
        .make_box()
    }
}
