use super::prelude::*;
use serde::Serialize;
use syn::{visit, Visibility};

use util::Hist;
use util::Monoid;

#[derive(Default, Serialize, Clone)]
pub struct Structs {
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
                .filter(|field| match field.vis {
                    Visibility::Public(_) => true,
                    _ => false,
                })
                .count(),
        );
        self.field_attrs_hist
            .observe(i.fields.iter().map(|x| x.attrs.len()).sum());
        self.attrs_hist.observe(i.attrs.len());
        visit::visit_item_struct(self, i);
    }
}

impl Visitor for Structs {
    fn visitor() -> MetricCollectorBox {
        util::VisitorCollector::new(
            "structs_metrics",
            Self::default(),
            |v| v,
            |v| util::Monoid::reduce(v.into_iter().cloned()),
        )
        .make_box()
    }
}

#[derive(Default, Serialize, Clone)]
pub struct Enums {
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
        visit::visit_item_enum(self, i);
    }
}

impl Visitor for Enums {
    fn visitor() -> MetricCollectorBox {
        util::VisitorCollector::new(
            "enum_metrics",
            Self::default(),
            |v| v,
            |v| util::Monoid::reduce(v.into_iter().cloned()),
        )
        .make_box()
    }
}

// histogram!("field_mutability_hist", FieldMutability<128>, visit_attribute, syn::Attribute, |i| ) //
// macro_rules! histogram {
//     ($name:literal, $struct:ident<$num_buckets:literal>, $hook:ident, $syn_struct:ident, $extract:expr) => {
//         #[derive(Default, Serialize)]
//         pub(crate) struct $struct(Hist<$num_buckets>);
//
//         impl Visit<'_> for $struct {
//             fn $hook(&mut self, i: &'_ $syn_struct) {
//                 let extract = $extract;
//                 self.0.observe(extract(i));
//                 visit::$hook(self, i);
//             }
//         }
//
//         impl Visitor for $struct {
//             fn visitor() -> MetricCollectorBox {
//                 util::VisitorCollector::new(
//                     $name
//                     Self::default(),
//                     |$struct(hist)| hist,
//                     |v| util::Monoid::reduce(v.into_iter.cloned()),
//                 )
//                 .make_box()
//             }
//         }
//     };
// }
