use super::prelude::*;
use average::{Estimate, Mean};
use syn::{Block, Expr, ExprClosure, ImplItemFn, ItemFn};

#[derive(Default)]
struct VisitorAvgMethodDepth {
    current_depth: u32,
    max_depth: u32,
    estimator: Mean,
}

impl VisitorAvgMethodDepth {
    fn handle_depth(&mut self, inner: impl FnOnce(&mut Self)) {
        let start_depth = self.current_depth;
        // reset max_depth for this function
        let old_max_depth = self.max_depth;
        self.max_depth = start_depth;

        inner(self);

        let depth = self.max_depth - start_depth;
        assert_ne!(depth, 0, "depth should never be 0");
        self.estimator.add(depth as f64);
        self.max_depth = old_max_depth;
    }

    fn add_depth(&mut self) {
        self.current_depth += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
    }

    fn sub_depth(&mut self) {
        self.current_depth -= 1;
    }
}

impl Visit<'_> for VisitorAvgMethodDepth {
    fn visit_block(&mut self, i: &'_ Block) {
        self.add_depth();
        syn::visit::visit_block(self, i);
        self.sub_depth();
    }

    fn visit_expr_closure(&mut self, i: &'_ ExprClosure) {
        self.handle_depth(|v| {
            // add a fake block to the closure if it doesn't have one
            let add_fake_block = !matches!(i.body.as_ref(), Expr::Block(_));
            if add_fake_block {
                v.add_depth();
            }
            syn::visit::visit_expr_closure(v, i);
            if add_fake_block {
                v.sub_depth();
            }
        });
    }

    fn visit_impl_item_fn(&mut self, i: &'_ ImplItemFn) {
        self.handle_depth(|v| syn::visit::visit_impl_item_fn(v, i));
    }

    fn visit_item_fn(&mut self, i: &'_ ItemFn) {
        self.handle_depth(|v| syn::visit::visit_item_fn(v, i));
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "avg_fn_depth",
        VisitorAvgMethodDepth::default(),
        |v| v.estimator,
        |v| util::merge_all(v).mean(),
    )
    .make_box()
}
