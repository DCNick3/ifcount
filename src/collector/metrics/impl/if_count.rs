use super::prelude::*;
use syn::ExprIf;

#[derive(Default)]
struct VisitorIfCount {
    ifcount: u64,
}

impl Visit<'_> for VisitorIfCount {
    fn visit_expr_if(&mut self, i: &'_ ExprIf) {
        self.ifcount += 1;
        syn::visit::visit_expr_if(self, i);
    }
}

pub fn visitor() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "if_count",
        VisitorIfCount::default(),
        |v| v.ifcount,
        |v| v.iter().sum::<u64>(),
    )
    .make_box()
}
