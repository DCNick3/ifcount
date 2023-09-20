use super::prelude::*;
use serde::Serialize;
use syn::visit;

use util::Hist;

#[derive(Default, Serialize)]
pub struct StructFields(Hist<64>);

impl Visit<'_> for StructFields {
    fn visit_item_struct(&mut self, i: &'_ syn::ItemStruct) {
        self.0.observe(i.fields.len());
        visit::visit_item_struct(self, i);
    }
}

impl Visitor for StructFields {
    fn visitor() -> MetricCollectorBox {
        util::VisitorCollector::new(
            "struct_fields_hist",
            Self::default(),
            |StructFields(hist)| hist,
            |v| util::Monoid::reduce(v.into_iter().cloned()),
        )
        .make_box()
    }
}

#[derive(Default, Serialize)]
pub struct EnumVariants(Hist<64>);

impl Visit<'_> for EnumVariants {
    fn visit_item_enum(&mut self, i: &'_ syn::ItemEnum) {
        self.0.observe(i.variants.len());
        visit::visit_item_enum(self, i);
    }
}

impl Visitor for EnumVariants {
    fn visitor() -> MetricCollectorBox {
        util::VisitorCollector::new(
            "enum_variants_hist",
            Self::default(),
            |EnumVariants(hist)| hist,
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
