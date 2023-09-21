use super::prelude::*;
use syn::Visibility;
use util::{Hist, Monoid};

#[derive(Default, Serialize, Clone)]
struct Structs {
    fields_hist: Hist<64>,
    public_fields_hist: Hist<64>,
    attrs_hist: Hist<32>,
    field_attrs_hist: Hist<32>,
}

impl Monoid for Structs {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            fields_hist: self.fields_hist + rhs.fields_hist,
            public_fields_hist: self.public_fields_hist + rhs.public_fields_hist,
            attrs_hist: self.attrs_hist + rhs.attrs_hist,
            field_attrs_hist: self.field_attrs_hist + rhs.field_attrs_hist,
        }
    }
}

impl Visit<'_> for Structs {
    fn visit_item_struct(&mut self, i: &'_ syn::ItemStruct) {
        self.fields_hist.observe(i.fields.len());
        self.public_fields_hist.observe(
            i.fields
                .iter()
                .filter(|field| matches!(field.vis, Visibility::Public(_)))
                .count(),
        );
        self.field_attrs_hist
            .observe(i.fields.iter().map(|x| x.attrs.len()).sum());
        self.attrs_hist.observe(i.attrs.len());
        syn::visit::visit_item_struct(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "structs_metrics",
        Structs::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}
