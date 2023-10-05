use syn::visit::{self, Visit};

use super::prelude::*;
use util::{Hist, Monoid};

#[derive(Default)]
pub struct StatementSize {
    expr_count: usize,
    hist: Hist<16>,
}

impl Visit<'_> for StatementSize {
    fn visit_expr(&mut self, i: &'_ syn::Expr) {
        self.expr_count += 1;

        visit::visit_expr(self, i);
    }

    fn visit_stmt(&mut self, i: &'_ syn::Stmt) {
        let old_expr_count = self.expr_count;
        self.expr_count = 0;

        visit::visit_stmt(self, i);

        self.hist.observe(self.expr_count);
        self.expr_count = old_expr_count;
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "statement_size",
        StatementSize::default(),
        |v| v,
        |v: &[StatementSize]| Monoid::reduce(v.iter().map(|v| v.hist.to_owned())),
    )
    .make_box()
}
