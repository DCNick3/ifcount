use super::prelude::{util::Observer, *};
use syn::Visibility;
use util::{Monoid, Unaggregated};

#[derive(Default, Serialize, Clone)]
struct Structs<Obs = Unaggregated> {
    fields_count: Obs,
    public_fields_count: Obs,
    attrs_count: Obs,
    field_attr_count: Obs,
}

impl<T: Monoid + Default> Monoid for Structs<T> {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            fields_count: self.fields_count.unite(rhs.fields_count),
            public_fields_count: self.public_fields_count.unite(rhs.public_fields_count),
            attrs_count: self.attrs_count.unite(rhs.attrs_count),
            field_attr_count: self.field_attr_count.unite(rhs.field_attr_count),
        }
    }
}

impl<Obs: Observer> Visit<'_> for Structs<Obs> {
    fn visit_item_struct(&mut self, i: &'_ syn::ItemStruct) {
        self.fields_count.observe(i.fields.len());
        self.public_fields_count.observe(
            i.fields
                .iter()
                .filter(|field| matches!(field.vis, Visibility::Public(_)))
                .count(),
        );
        i.fields.iter().for_each(|x| {
            self.field_attr_count.observe(x.attrs.len());
        });
        self.attrs_count.observe(i.attrs.len());
        syn::visit::visit_item_struct(self, i);
    }
}

pub fn make_collector<
    Obs: Observer + Default + Serialize + Clone + Monoid + Send + Sync + 'static,
>() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "structs",
        Structs::<Obs>::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}

#[cfg(test)]
mod tests {
    use super::Structs;
    use crate::collector::metrics::util::check;
    use expect_test::expect;
    use syn::parse_quote;

    #[test]
    fn struct_metrics() {
        let code = parse_quote! {
            #[derive(Debug)]
            struct Thing {
                u: i32,
            }

            #[derive(Debug)]
            #[allow(dead_code)]
            struct Thing2 {
                pub u: i32,
                pub v: u32,
                #[allow(dead_code)]
                z: (),
            }
        };
        check::<Structs>(
            code,
            expect![[r#"
                {
                  "fields_count": [
                    1,
                    3
                  ],
                  "public_fields_count": [
                    0,
                    2
                  ],
                  "attrs_count": [
                    1,
                    2
                  ],
                  "field_attr_count": [
                    0,
                    0,
                    0,
                    1
                  ]
                }"#]],
        );
    }

    #[test]
    fn count_file() {
        let code = parse_quote! {
            struct A;

            #[derive(Debug)]
            struct Name {
                #[attr1]
                #[attr2]
                field: Type
            }

            #[derive(Debug)]
            enum Name {
                Variant1,
                Variant2,
            }

            struct Tuple(A);
            pub struct Public {
                pub f1: Ty1,
                f2: Ty2,
                f3: Ty3,
            }

            struct TupleWithPubs(pub i32, pub Box<Type>, String);
        };
        check::<Structs>(
            code,
            expect![[r#"
                {
                  "fields_count": [
                    0,
                    1,
                    1,
                    3,
                    3
                  ],
                  "public_fields_count": [
                    0,
                    0,
                    0,
                    1,
                    2
                  ],
                  "attrs_count": [
                    0,
                    1,
                    0,
                    0,
                    0
                  ],
                  "field_attr_count": [
                    2,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0
                  ]
                }"#]],
        )
    }
}
