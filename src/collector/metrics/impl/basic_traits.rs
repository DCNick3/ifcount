use super::prelude::*;
use syn::TraitItem;
use util::{Hist, Monoid};

#[derive(Default, Clone, Serialize)]
struct TraitDefinitions {
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
                    TraitItem::Fn(function) => function.default.is_some(),
                    _ => false,
                })
                .count(),
        );
        self.all_fn_hist.observe(
            i.items
                .iter()
                .filter(|trait_item| matches!(trait_item, TraitItem::Fn(_)))
                .count(),
        );
        self.assoc_types_hist.observe(
            i.items
                .iter()
                .filter(|trait_item| matches!(trait_item, TraitItem::Type(_)))
                .count(),
        )
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "trait_definition_metrics",
        TraitDefinitions::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}
