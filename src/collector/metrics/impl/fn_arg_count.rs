use syn::visit::{self, Visit};

use super::prelude::*;
use util::{Hist, Monoid};

#[derive(Default)]
pub struct FnArgsCount(Hist);

impl Visit<'_> for FnArgsCount {
    fn visit_signature(&mut self, i: &'_ syn::Signature) {
        let arg_count = i.inputs.len();
        self.0.observe(arg_count);
        visit::visit_signature(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "fn_arg_count",
        FnArgsCount::default(),
        |v| v,
        |v: &[FnArgsCount]| Monoid::reduce(v.iter().map(|FnArgsCount(hist)| hist.to_owned())),
    )
    .make_box()
}
