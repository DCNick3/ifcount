use syn::visit::{self, Visit};

use super::prelude::*;
use util::{Hist, Monoid};

#[derive(Default)]
pub struct FnArgsHist(Hist<16>);

impl Visit<'_> for FnArgsHist {
    fn visit_signature(&mut self, i: &'_ syn::Signature) {
        let arg_count = i.inputs.len();
        self.0.observe(arg_count);
        visit::visit_signature(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "fn_arg_hist",
        FnArgsHist::default(),
        |v| v,
        |v: &[FnArgsHist]| Monoid::reduce(v.into_iter().map(|FnArgsHist(hist)| hist.to_owned())),
    )
    .make_box()
}
