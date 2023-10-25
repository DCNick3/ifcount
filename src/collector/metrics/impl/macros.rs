use super::prelude::{
    util::{Observer, Unaggregated},
    *,
};
use util::Monoid;

#[derive(Default, Clone, Serialize)]
struct MacroStats<Obs = Unaggregated> {
    argument_size: Obs,
    count_per_file: Obs,
}

impl<T: Monoid + Default> Monoid for MacroStats<T> {
    fn init() -> Self {
        Self::default()
    }
    fn unite(self, rhs: Self) -> Self {
        Self {
            argument_size: self.argument_size.unite(rhs.argument_size),
            count_per_file: self.count_per_file.unite(rhs.count_per_file),
        }
    }
}

impl<Obs: Observer> Visit<'_> for MacroStats<Obs> {
    fn visit_file(&mut self, i: &'_ syn::File) {
        let start_count = self.argument_size.count();
        syn::visit::visit_file(self, i);
        let end_count = self.argument_size.count();

        self.count_per_file
            .observe((end_count - start_count) as usize);
    }

    fn visit_macro(&mut self, i: &'_ syn::Macro) {
        self.argument_size
            .observe(i.tokens.clone().into_iter().count());
    }
}

pub fn make_collector<
    Obs: Observer + Default + Serialize + Clone + Monoid + Send + Sync + 'static,
>() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "macro",
        MacroStats::<Obs>::default(),
        |v| v,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}

#[cfg(test)]
mod tests {
    use super::MacroStats;
    use crate::collector::metrics::util::check;
    use expect_test::expect;
    use syn::parse_quote;

    #[test]
    fn test_macro_stats() {
        let code = parse_quote! {
            macro_rules! foo {
                ($($x:tt)*) => {
                    $($x)*
                }
            }
        };

        check::<MacroStats>(
            code,
            expect![[r#"
                {
                  "argument_size": [
                    4
                  ],
                  "count_per_file": [
                    1
                  ]
                }"#]],
        )
    }

    #[test]
    fn test_more_macros() {
        let code = parse_quote! {
            impl_num!(u32);
            impl_num!(u64);
            impl_num!(u128);
        };

        check::<MacroStats>(
            code,
            expect![[r#"
                {
                  "argument_size": [
                    1,
                    1,
                    1
                  ],
                  "count_per_file": [
                    3
                  ]
                }"#]],
        )
    }

    #[test]
    fn test_big_call() {
        let code = parse_quote! {
            do_stuff! {
                // (only token groups are counted, not individual tokens)
                //1    2     3   4
                pub struct Hello {
                    pub field: Type,
                }
            }
        };

        check::<MacroStats>(
            code,
            expect![[r#"
                {
                  "argument_size": [
                    4
                  ],
                  "count_per_file": [
                    1
                  ]
                }"#]],
        )
    }
}
