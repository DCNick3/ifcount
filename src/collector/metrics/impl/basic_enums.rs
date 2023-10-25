use super::prelude::{util::Observer, *};
use util::{Monoid, Unaggregated};

#[derive(Default, Serialize, Clone)]
struct Enums<Obs = Unaggregated> {
    variant_count: Obs,
    attr_count: Obs,
    variant_attr_count: Obs,
}

impl<T: Monoid + Default> Monoid for Enums<T> {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            variant_count: self.variant_count.unite(rhs.variant_count),
            attr_count: self.attr_count.unite(rhs.attr_count),
            variant_attr_count: self.variant_attr_count.unite(rhs.variant_attr_count),
        }
    }
}

impl<Obs: Observer> Visit<'_> for Enums<Obs> {
    fn visit_item_enum(&mut self, i: &'_ syn::ItemEnum) {
        self.variant_count.observe(i.variants.len());
        self.attr_count.observe(i.attrs.len());
        i.variants
            .iter()
            .for_each(|x| self.variant_attr_count.observe(x.attrs.len()));
        syn::visit::visit_item_enum(self, i);
    }
}

pub fn make_collector<
    Obs: Observer + Default + Serialize + Clone + Monoid + Send + Sync + 'static,
>() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "enums",
        Enums::<Obs>::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}

#[cfg(test)]
mod tests {
    use super::Enums;
    use crate::collector::metrics::util::check;
    use expect_test::expect;
    use syn::parse_quote;

    #[test]
    fn no_enums() {
        let code = parse_quote! {
            struct Thing {
                u: i32,
            }
        };
        check::<Enums>(
            code,
            expect![[r#"
                {
                  "variant_count": [],
                  "attr_count": [],
                  "variant_attr_count": []
                }"#]],
        );
    }

    #[test]
    fn small_enums() {
        let code = parse_quote! {
            #[derive(Debug, Clone, Copy)]
            enum SmallEnum {
                A,
                B,
                C,
            }

            #[derive(Debug, Clone)]
            enum SmallEnum {
                A,
                B,
            }
        };
        check::<Enums>(
            code,
            expect![[r#"
                {
                  "variant_count": [
                    3,
                    2
                  ],
                  "attr_count": [
                    1,
                    1
                  ],
                  "variant_attr_count": [
                    0,
                    0,
                    0,
                    0,
                    0
                  ]
                }"#]],
        );
    }

    #[test]
    fn big_enum() {
        let code = parse_quote! {
            #[derive(Debug, Clone, Copy)]
            #[serde(tag = "type")]
            enum BigEnum {
                #[serde(rename = "a")]
                A,
                #[serde(rename = "b")]
                B,
                C,
                D,
                E,
                F
            }
        };
        check::<Enums>(
            code,
            expect![[r#"
                {
                  "variant_count": [
                    6
                  ],
                  "attr_count": [
                    2
                  ],
                  "variant_attr_count": [
                    1,
                    1,
                    0,
                    0,
                    0,
                    0
                  ]
                }"#]],
        );
    }
}
