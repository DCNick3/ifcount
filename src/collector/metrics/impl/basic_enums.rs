use super::prelude::*;
use util::{Hist, Monoid};

#[derive(Default, Serialize, Clone)]
struct Enums {
    variant_count: Hist<64>,
    attr_count: Hist<64>,
    variant_attr_count: Hist<64>,
}

impl Monoid for Enums {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            variant_count: self.variant_count + rhs.variant_count,
            attr_count: self.attr_count + rhs.attr_count,
            variant_attr_count: self.variant_attr_count + rhs.variant_attr_count,
        }
    }
}

impl Visit<'_> for Enums {
    fn visit_item_enum(&mut self, i: &'_ syn::ItemEnum) {
        self.variant_count.observe(i.variants.len());
        self.attr_count.observe(i.attrs.len());
        self.variant_attr_count
            .observe(i.variants.iter().map(|x| x.attrs.len()).sum());
        syn::visit::visit_item_enum(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "enums",
        Enums::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}
