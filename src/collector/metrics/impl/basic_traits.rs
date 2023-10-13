use super::prelude::*;
use syn::TraitItem;
use util::{Hist, Monoid};

#[derive(Default, Clone, Serialize)]
struct TraitDefinitions {
    generic_param_count: Hist,
    supertrait_count: Hist,
    default_fn_count: Hist,
    all_fn_count: Hist,
    assoc_type_count: Hist,
}

impl Monoid for TraitDefinitions {
    fn init() -> Self {
        Self::default()
    }
    fn unite(self, rhs: Self) -> Self {
        Self {
            generic_param_count: self.generic_param_count + rhs.generic_param_count,
            supertrait_count: self.supertrait_count + rhs.supertrait_count,
            default_fn_count: self.default_fn_count + rhs.default_fn_count,
            all_fn_count: self.all_fn_count + rhs.all_fn_count,
            assoc_type_count: self.assoc_type_count + rhs.assoc_type_count,
        }
    }
}

impl Visit<'_> for TraitDefinitions {
    fn visit_item_trait(&mut self, i: &'_ syn::ItemTrait) {
        self.generic_param_count.observe(i.generics.params.len());
        self.supertrait_count.observe(i.supertraits.len());
        self.default_fn_count.observe(
            i.items
                .iter()
                .filter(|trait_item| match trait_item {
                    TraitItem::Fn(function) => function.default.is_some(),
                    _ => false,
                })
                .count(),
        );
        self.all_fn_count.observe(
            i.items
                .iter()
                .filter(|trait_item| matches!(trait_item, TraitItem::Fn(_)))
                .count(),
        );
        self.assoc_type_count.observe(
            i.items
                .iter()
                .filter(|trait_item| matches!(trait_item, TraitItem::Type(_)))
                .count(),
        );
        syn::visit::visit_item_trait(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "trait_def",
        TraitDefinitions::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}
