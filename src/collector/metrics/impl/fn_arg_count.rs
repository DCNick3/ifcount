use syn::{
    visit::{self, Visit},
    FnArg, PatType, Type,
};

use super::prelude::*;
use util::{Hist, Monoid};

#[derive(Default, Clone, Serialize)]
pub struct FnArgsCount {
    mutable: Hist,
}

impl Monoid for FnArgsCount {
    fn init() -> Self {
        Self::default()
    }

    fn unite(self, rhs: Self) -> Self {
        Self {
            mutable: self.mutable.unite(rhs.mutable),
        }
    }
}

impl Visit<'_> for FnArgsCount {
    fn visit_signature(&mut self, i: &'_ syn::Signature) {
        let mutable = i
            .inputs
            .iter()
            .filter(|arg| {
                match arg {
                    // only count mutable references, mut by move does not affect function
                    // interface
                    FnArg::Receiver(arg) => arg.mutability.is_some() && arg.reference.is_some(),
                    FnArg::Typed(PatType { ty, .. }) => {
                        let ty: &Type = &ty; // Box matching :clown_emoji:
                        match ty {
                            Type::Reference(reference) => reference.mutability.is_some(),
                            _ => false,
                        }
                    }
                }
            })
            .count();

        self.mutable.observe(mutable);

        visit::visit_signature(self, i);
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "fn_arg_count",
        FnArgsCount::default(),
        |v| v,
        |v: &[FnArgsCount]| Monoid::reduce(v.into_iter().map(|args| args.to_owned())),
    )
    .make_box()
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use syn::parse_quote;

    use crate::collector::metrics::util::check;

    use super::FnArgsCount;

    #[test]
    fn refs() {
        let code = parse_quote! {
                    fn a(
                        &mut arg: &mut Type, // 1
        //                ^ pattern
                        arg2: &mut Type2 // 2
                        ) {
                        todo!()
                    }

                    impl T {
                        fn method(
                            &mut self, // 3
                            &mut other: &mut Self // 4
                            ){}
                    }
                };
        check::<FnArgsCount>(code, expect![[r#"
            {
              "mutable": {
                "sum": 4,
                "avg": 2.0,
                "mode": 2
              }
            }"#]]);
    }
}
