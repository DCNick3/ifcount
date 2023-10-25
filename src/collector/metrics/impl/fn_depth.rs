use super::prelude::{
    util::{Observer, Unaggregated},
    *,
};
use crate::collector::metrics::util::Monoid;
use syn::{Block, Expr, ExprClosure, ImplItemFn, ItemFn};

#[derive(Default)]
struct VisitorAvgMethodDepth<Obs = Unaggregated> {
    current_depth: u32,
    max_depth: u32,
    observer: Obs,
}

impl<Obs: Observer> VisitorAvgMethodDepth<Obs> {
    fn handle_depth(&mut self, inner: impl FnOnce(&mut Self)) {
        let start_depth = self.current_depth;
        // reset max_depth for this function
        let old_max_depth = self.max_depth;
        self.max_depth = start_depth;

        inner(self);

        let depth = self.max_depth - start_depth;
        assert_ne!(depth, 0, "depth should never be 0");
        self.observer.observe(depth as usize);
        self.max_depth = old_max_depth;
    }

    fn add_depth(&mut self) {
        self.current_depth += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
    }

    fn sub_depth(&mut self) {
        self.current_depth -= 1;
    }
}

impl<Obs: Observer> Visit<'_> for VisitorAvgMethodDepth<Obs> {
    fn visit_block(&mut self, i: &'_ Block) {
        self.add_depth();
        syn::visit::visit_block(self, i);
        self.sub_depth();
    }

    fn visit_expr_closure(&mut self, i: &'_ ExprClosure) {
        self.handle_depth(|v| {
            // add a fake block to the closure if it doesn't have one
            let add_fake_block = !matches!(i.body.as_ref(), Expr::Block(_));
            if add_fake_block {
                v.add_depth();
            }
            syn::visit::visit_expr_closure(v, i);
            if add_fake_block {
                v.sub_depth();
            }
        });
    }

    fn visit_impl_item_fn(&mut self, i: &'_ ImplItemFn) {
        self.handle_depth(|v| syn::visit::visit_impl_item_fn(v, i));
    }

    fn visit_item_fn(&mut self, i: &'_ ItemFn) {
        self.handle_depth(|v| syn::visit::visit_item_fn(v, i));
    }
}

pub fn make_collector<
    Obs: Observer + Default + Serialize + Clone + Monoid + Send + Sync + 'static,
>() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "fn_depth",
        VisitorAvgMethodDepth::<Obs>::default(),
        |v| v.observer,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}

#[cfg(test)]
mod tests {
    use crate::collector::metrics::util::Unaggregated;

    use super::VisitorAvgMethodDepth;
    use expect_test::{expect, Expect};
    use syn::parse_quote;
    use syn::visit::Visit;

    fn check(code: syn::File, expect: Expect) {
        let mut metric = VisitorAvgMethodDepth::<Unaggregated>::default();
        metric.visit_file(&code);
        let metric = serde_json::to_string(&metric.observer).unwrap();
        expect.assert_eq(&metric)
    }

    #[test]
    fn test_depth() {
        check(
            parse_quote! {
                fn foo() {
                    let x = 1;
                    let y = 2;
                    let z = 3;
                }
            },
            expect![["[1]"]],
        );
        check(
            parse_quote! {
                fn foo() {
                    {}
                }
            },
            expect![["[2]"]],
        );
        check(
            parse_quote! {
                fn foo() {
                    // closures are their own scope
                    || {}
                }
            },
            expect![["[1,1]"]],
        );
        check(
            parse_quote! {
                fn foo() {
                    struct A;
                    impl A {
                        fn inner() {
                            {}
                        }
                    }
                }
            },
            expect!["[2,1]"],
        );
        check(
            parse_quote! {
                fn foo() {
                    if true {
                        println!("hello");

                        if true {
                            while 10 < 19 {
                                println!("hello");
                            }

                            loop {
                                println!("hello");
                                if false {
                                    break;
                                }
                            }
                        }

                    } else {
                        for i in 0..10 {
                            println!("hello");
                        }
                    }
                }
            },
            expect![["[5]"]],
        );
    }
}
