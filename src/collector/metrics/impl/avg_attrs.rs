use super::prelude::*;
use serde::Serialize;
use syn::TraitItem;
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

#[derive(Default, Clone, Serialize)]
pub struct TraitDefinitions {
    generic_params_num_hist: Hist<16>,
    supertraits_num_hist: Hist<16>,
    default_fn_hist: Hist<128>,
    all_fn_hist: Hist<256>,
    assoc_types_hist: Hist<128>,
}

impl Monoid for TraitDefinitions {
    fn init() -> Self {
        Self::default()
    }
    fn unite(self, rhs: Self) -> Self {
        Self {
            generic_params_num_hist: self.generic_params_num_hist + rhs.generic_params_num_hist,
            supertraits_num_hist: self.supertraits_num_hist + rhs.supertraits_num_hist,
            default_fn_hist: self.default_fn_hist + rhs.default_fn_hist,
            all_fn_hist: self.all_fn_hist + rhs.all_fn_hist,
            assoc_types_hist: self.assoc_types_hist + rhs.assoc_types_hist,
        }
    }
}

impl Visit<'_> for TraitDefinitions {
    fn visit_item_trait(&mut self, i: &'_ syn::ItemTrait) {
        self.generic_params_num_hist
            .observe(i.generics.params.len());
        self.supertraits_num_hist.observe(i.supertraits.len());
        self.default_fn_hist.observe(
            i.items
                .iter()
                .filter(|trait_item| match trait_item {
                    TraitItem::Fn(funtion) => funtion.default.is_some(),
                    _ => false,
                })
                .count(),
        );
        self.all_fn_hist.observe(
            i.items
                .iter()
                .filter(|trait_item| match trait_item {
                    TraitItem::Fn(_) => true,
                    _ => false,
                })
                .count(),
        );
        self.assoc_types_hist.observe(
            i.items
                .iter()
                .filter(|trait_item| match trait_item {
                    TraitItem::Type(_) => true,
                    _ => false,
                })
                .count(),
        )
    }
}

impl Visitor for TraitDefinitions {
    fn visitor() -> MetricCollectorBox {
        util::VisitorCollector::new(
            "trait_definition_metrics",
            Self::default(),
            |v| v,
            |v| util::Monoid::reduce(v.into_iter().cloned()),
        )
        .make_box()
    }
}
