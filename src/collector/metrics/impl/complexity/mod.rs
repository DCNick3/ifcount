//! Evaluates cognitive complexity on a function level.

mod r#impl;

use super::prelude::*;
use syn::{visit, ExprClosure, ImplItemFn, ItemFn};
use util::{Hist, Monoid};

#[derive(Default, Clone, Serialize)]
struct ComplexityStats {
    item_fn: Hist,
    impl_item_fn: Hist,
    closure: Hist,
}

impl Monoid for ComplexityStats {
    fn init() -> Self {
        Self::default()
    }
    fn unite(self, rhs: Self) -> Self {
        Self {
            item_fn: self.item_fn + rhs.item_fn,
            impl_item_fn: self.impl_item_fn + rhs.impl_item_fn,
            closure: self.closure + rhs.closure,
        }
    }
}

impl Visit<'_> for ComplexityStats {
    fn visit_expr_closure(&mut self, i: &'_ ExprClosure) {
        self.closure
            .observe(r#impl::eval_expr(&i.body, Default::default()).0 as usize);
        visit::visit_expr_closure(self, i);
    }

    fn visit_impl_item_fn(&mut self, i: &'_ ImplItemFn) {
        self.impl_item_fn
            .observe(r#impl::eval_block(&i.block, Default::default()).0 as usize);
        visit::visit_impl_item_fn(self, i);
    }

    fn visit_item_fn(&mut self, i: &'_ ItemFn) {
        self.item_fn
            .observe(r#impl::eval_block(&i.block, Default::default()).0 as usize);
        visit::visit_item_fn(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "complexity",
        ComplexityStats::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}
