use super::prelude::*;
use average::{Estimate, Mean};
use syn::visit;

#[derive(Default)]
pub struct AvgStructAttrsCount {
    estimator: Mean,
}

impl Visit<'_> for AvgStructAttrsCount {
    fn visit_item_struct(&mut self, i: &'_ syn::ItemStruct) {
        self.estimator.add(i.fields.len() as f64);
        visit::visit_item_struct(self, i);
    }
}

impl Visitor for AvgStructAttrsCount {
    fn visitor() -> MetricCollectorBox {
        util::VisitorCollector::new(
            "struct_attrs_avg",
            Self::default(),
            |v| v.estimator,
            |v| util::merge_all(v).mean(),
        )
        .make_box()
    }
}

#[derive(Default)]
pub struct AvgEnumVariantsCount {
    estimator: Mean,
}

impl Visit<'_> for AvgEnumVariantsCount {
    fn visit_item_enum(&mut self, i: &'_ syn::ItemEnum) {
        self.estimator.add(i.variants.len() as f64);
        visit::visit_item_enum(self, i);
    }
}

impl Visitor for AvgEnumVariantsCount {
    fn visitor() -> MetricCollectorBox {
        util::VisitorCollector::new(
            "enum_variants_avg",
            Self::default(),
            |v| v.estimator,
            |v| util::merge_all(v).mean(),
        )
        .make_box()
    }
}
