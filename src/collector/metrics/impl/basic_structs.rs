use super::prelude::*;
use syn::Visibility;
use util::{Monoid, Unaggregated};

#[derive(Default, Serialize, Clone)]
struct Structs {
    fields_count: Unaggregated,
    public_fields_count: Unaggregated,
    attrs_count: Unaggregated,
    field_attr_count: Unaggregated,
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
        i.fields.iter().for_each(|x| {
            self.field_attr_count.observe(x.attrs.len());
        });
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
              "fields_count": {
                "sum": 4,
                "avg": 2.0,
                "mode": 3
              },
              "public_fields_count": {
                "sum": 2,
                "avg": 1.0,
                "mode": 2
              },
              "attrs_count": {
                "sum": 3,
                "avg": 1.5,
                "mode": 2
              },
              "field_attr_count": {
                "sum": 1,
                "avg": 0.25,
                "mode": 0
              }
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
              "fields_count": {
                "sum": 8,
                "avg": 1.6,
                "mode": 3
              },
              "public_fields_count": {
                "sum": 3,
                "avg": 0.6,
                "mode": 0
              },
              "attrs_count": {
                "sum": 1,
                "avg": 0.2,
                "mode": 0
              },
              "field_attr_count": {
                "sum": 2,
                "avg": 0.25,
                "mode": 0
              }
            }"#]],
        )
    }
}
