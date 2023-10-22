//! Evaluates cognitive complexity on a function level.

mod r#impl;

use super::prelude::*;
use syn::{visit, ExprClosure, ImplItemFn, ItemFn};
use util::Monoid;
use util::Observer;

#[derive(Default, Clone, Serialize)]
struct ComplexityStats<Obs> {
    item_fn: Obs,
    impl_item_fn: Obs,
    closure: Obs,
}

impl<T: Monoid> Monoid for ComplexityStats<T> {
    fn init() -> Self {
        Self {
            item_fn: Monoid::init(),
            impl_item_fn: Monoid::init(),
            closure: Monoid::init(),
        }
    }
    fn unite(self, rhs: Self) -> Self {
        Self {
            item_fn: self.item_fn.unite(rhs.item_fn),
            impl_item_fn: self.impl_item_fn.unite(rhs.impl_item_fn),
            closure: self.closure.unite(rhs.closure),
        }
    }
}

impl<Obs: Observer> Visit<'_> for ComplexityStats<Obs> {
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

pub fn make_collector<Obs: Observer + Default + Serialize + Clone + Monoid + Send + Sync>(
) -> MetricCollectorBox {
    util::VisitorCollector::new(
        "complexity",
        ComplexityStats::<Obs>::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box::<Obs>()
}
