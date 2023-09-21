use super::prelude::*;
use util::{Hist, Monoid};

#[derive(Default, Serialize, Clone)]
struct Enums {
    variants_hist: Hist<64>,
    attrs_hist: Hist<64>,
    variant_attrs_hist: Hist<64>,
}

impl Monoid for Enums {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            variants_hist: self.variants_hist + rhs.variants_hist,
            attrs_hist: self.attrs_hist + rhs.attrs_hist,
            variant_attrs_hist: self.variant_attrs_hist + rhs.variant_attrs_hist,
        }
    }
}

impl Visit<'_> for Enums {
    fn visit_item_enum(&mut self, i: &'_ syn::ItemEnum) {
        self.variants_hist.observe(i.variants.len());
        self.attrs_hist.observe(i.attrs.len());
        self.variant_attrs_hist
            .observe(i.variants.iter().map(|x| x.attrs.len()).sum());
        syn::visit::visit_item_enum(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "enum_metrics",
        Enums::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}
