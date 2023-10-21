use super::prelude::*;
use syn::TraitItem;
use util::{Monoid, Unaggregated};

#[derive(Default, Clone, Serialize)]
struct TraitDefinitions {
    generic_param_count: Unaggregated,
    supertrait_count: Unaggregated,
    default_fn_count: Unaggregated,
    all_fn_count: Unaggregated,
    assoc_type_count: Unaggregated,
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

#[cfg(test)]
mod tests {
    use super::TraitDefinitions;
    use crate::collector::metrics::util::check;
    use expect_test::expect;
    use syn::parse_quote;

    #[test]
    fn no_traits() {
        let code = parse_quote! {
            struct Thing {
                u: i32,
            }
        };
        check::<TraitDefinitions>(
            code,
            expect![[r#"
                {
                  "generic_param_count": {
                    "sum": 0,
                    "avg": null,
                    "mode": null
                  },
                  "supertrait_count": {
                    "sum": 0,
                    "avg": null,
                    "mode": null
                  },
                  "default_fn_count": {
                    "sum": 0,
                    "avg": null,
                    "mode": null
                  },
                  "all_fn_count": {
                    "sum": 0,
                    "avg": null,
                    "mode": null
                  },
                  "assoc_type_count": {
                    "sum": 0,
                    "avg": null,
                    "mode": null
                  }
                }"#]],
        );
    }

    #[test]
    fn one_trait() {
        let code = parse_quote! {
            trait Thing {
                fn foo();
            }
        };
        check::<TraitDefinitions>(
            code,
            expect![[r#"
                {
                  "generic_param_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "supertrait_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "default_fn_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "all_fn_count": {
                    "sum": 1,
                    "avg": 1.0,
                    "mode": 1
                  },
                  "assoc_type_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  }
                }"#]],
        );
    }

    #[test]
    fn one_trait_with_supertraits_and_default_fns() {
        let code = parse_quote! {
            trait Thing: Clone + Copy {
                fn foo();
                fn foo2() {

                }
                fn foo3() {

                }
            }
        };
        check::<TraitDefinitions>(
            code,
            expect![[r#"
                {
                  "generic_param_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "supertrait_count": {
                    "sum": 2,
                    "avg": 2.0,
                    "mode": 2
                  },
                  "default_fn_count": {
                    "sum": 2,
                    "avg": 2.0,
                    "mode": 2
                  },
                  "all_fn_count": {
                    "sum": 3,
                    "avg": 3.0,
                    "mode": 3
                  },
                  "assoc_type_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  }
                }"#]],
        );
    }

    #[test]
    fn one_trait_with_assoc_types() {
        let code = parse_quote! {
            trait Thing {
                type A;
                type B;
            }
        };
        check::<TraitDefinitions>(
            code,
            expect![[r#"
                {
                  "generic_param_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "supertrait_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "default_fn_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "all_fn_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "assoc_type_count": {
                    "sum": 2,
                    "avg": 2.0,
                    "mode": 2
                  }
                }"#]],
        );
    }

    #[test]
    fn two_traits_with_generic_parameters() {
        let code = parse_quote! {
            trait Thing<T> {
                type A;
                type B;
            }
            trait Thing2<T, U> {
                type A;
                type B;
            }
        };
        check::<TraitDefinitions>(
            code,
            expect![[r#"
                {
                  "generic_param_count": {
                    "sum": 3,
                    "avg": 1.5,
                    "mode": 2
                  },
                  "supertrait_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "default_fn_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "all_fn_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "assoc_type_count": {
                    "sum": 4,
                    "avg": 2.0,
                    "mode": 2
                  }
                }"#]],
        );
    }

    #[test]
    fn one_trait_inside_function() {
        let code = parse_quote! {
            fn foo() {
                trait Thing {
                    type A;
                    type B;
                }
            }
        };
        check::<TraitDefinitions>(
            code,
            expect![[r#"
                {
                  "generic_param_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "supertrait_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "default_fn_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "all_fn_count": {
                    "sum": 0,
                    "avg": 0.0,
                    "mode": 0
                  },
                  "assoc_type_count": {
                    "sum": 2,
                    "avg": 2.0,
                    "mode": 2
                  }
                }"#]],
        );
    }
}
