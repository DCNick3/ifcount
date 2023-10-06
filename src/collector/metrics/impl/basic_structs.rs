use super::prelude::*;
use syn::Visibility;
use util::{Hist, Monoid};

#[derive(Default, Serialize, Clone)]
struct Structs {
    fields_count: Hist<64>,
    public_fields_count: Hist<64>,
    attrs_count: Hist<32>,
    field_attr_count: Hist<32>,
}

impl Monoid for Structs {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            fields_count: self.fields_count + rhs.fields_count,
            public_fields_count: self.public_fields_count + rhs.public_fields_count,
            attrs_count: self.attrs_count + rhs.attrs_count,
            field_attr_count: self.field_attr_count + rhs.field_attr_count,
        }
    }
}

impl Visit<'_> for Structs {
    fn visit_item_struct(&mut self, i: &'_ syn::ItemStruct) {
        self.fields_count.observe(i.fields.len());
        self.public_fields_count.observe(
            i.fields
                .iter()
                .filter(|field| matches!(field.vis, Visibility::Public(_)))
                .count(),
        );
        self.field_attr_count
            .observe(i.fields.iter().map(|x| x.attrs.len()).sum());
        self.attrs_count.observe(i.attrs.len());
        syn::visit::visit_item_struct(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "structs",
        Structs::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}
