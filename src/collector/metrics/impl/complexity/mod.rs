//! Evaluates cognitive complexity on a function level.

mod r#impl;

use super::prelude::*;
use syn::{visit, ExprClosure, ImplItemFn, ItemFn};
use util::{Hist, Monoid};

#[derive(Default, Clone, Serialize)]
struct ComplexityStats {
    item_fn_complexity: Hist<128>,
    impl_item_fn_complexity: Hist<128>,
    closure_complexity: Hist<128>,
}

impl Monoid for ComplexityStats {
    fn init() -> Self {
        Self::default()
    }
    fn unite(self, rhs: Self) -> Self {
        Self {
            item_fn_complexity: self.item_fn_complexity + rhs.item_fn_complexity,
            impl_item_fn_complexity: self.impl_item_fn_complexity + rhs.impl_item_fn_complexity,
            closure_complexity: self.closure_complexity + rhs.closure_complexity,
        }
    }
}

impl Visit<'_> for ComplexityStats {
    fn visit_expr_closure(&mut self, i: &'_ ExprClosure) {
        self.closure_complexity
            .observe(r#impl::eval_expr(&i.body, Default::default()).0 as usize);
        visit::visit_expr_closure(self, i);
    }

    fn visit_impl_item_fn(&mut self, i: &'_ ImplItemFn) {
        self.impl_item_fn_complexity
            .observe(r#impl::eval_block(&i.block, Default::default()).0 as usize);
        visit::visit_impl_item_fn(self, i);
    }

    fn visit_item_fn(&mut self, i: &'_ ItemFn) {
        self.item_fn_complexity
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
