use super::prelude::*;
use syn::ExprIf;

#[derive(Default)]
struct VisitorIfCount {
    ifcount: u64,
}

impl Visit<'_> for VisitorIfCount {
    fn visit_expr_if(&mut self, i: &'_ ExprIf) {
        self.ifcount += 1;
        syn::visit::visit_expr_if(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "if_count",
        VisitorIfCount::default(),
        |v| v.ifcount,
        |v| v.iter().sum::<u64>(),
    )
    .make_box()
}

#[cfg(test)]
mod tests {
    use super::VisitorIfCount;
    use expect_test::{expect, Expect};
    use syn::parse_quote;
    use syn::visit::Visit;

    fn check(code: syn::File, expect: Expect) {
        let mut metric = VisitorIfCount::default();
        metric.visit_file(&code);
        let metric = serde_json::to_string(&metric.ifcount).unwrap();
        expect.assert_eq(&metric)
    }

    #[test]
    fn test_ifcount() {
        check(
            parse_quote! {
                fn foo() {
                    let x = 1;
                    let y = 2;
                    let z = 3;
                }
            },
            expect![["0"]],
        );
        check(
            parse_quote! {
                fn foo() {
                    // closures are their own scope
                    || {
                        if true {}
                    }
                }
            },
            expect![["1"]],
        );
        check(
            parse_quote! {
                fn foo() {
                    struct A;
                    impl A {
                        fn inner() {
                            if false {} else if true {} else {}
                        }
                    }
                }
            },
            expect![["2"]],
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
            expect![["3"]],
        );
    }
}
