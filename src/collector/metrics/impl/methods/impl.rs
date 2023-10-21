use std::collections::HashMap;

use syn::{ImplItem, ImplItemFn};

use crate::collector::metrics::util::{Monoid, Unaggregated};

use super::{
    prelude::*,
    util::{FnFieldSet, FnMethodCalls},
};

/// lack of cohesion of methods per impl block
#[derive(Default, Serialize)]
struct ImplLcom4(Unaggregated);

impl Visit<'_> for ImplLcom4 {
    fn visit_item_impl(&mut self, i: &'_ syn::ItemImpl) {
        let mut fields = FnFieldSet::default();
        let mut calls = FnMethodCalls::default();
        syn::visit::visit_item_impl(&mut fields, i);
        syn::visit::visit_item_impl(&mut calls, i);

        let funcs: Vec<_> = i
            .items
            .iter()
            .flat_map(|x| match x {
                ImplItem::Fn(func) => Some(func),
                _ => None,
            })
            .collect();
        let func_pairs = funcs
            .iter()
            .flat_map(|&x| funcs.iter().map(move |&y| (x, y)))
            .filter(|(x, y)| x != y);
        let related: Vec<(&ImplItemFn, &ImplItemFn)> = func_pairs
            .filter(|(func1, func2)| {
                let by_fields = fields.related(func1, func2);

                let by_calls = calls.related(func1, func2);
                by_fields || by_calls
            })
            .collect();

        let mut neighbours: HashMap<&ImplItemFn, Vec<&ImplItemFn>> = HashMap::new();
        for func in funcs {
            neighbours.insert(func, vec![]);
        }
        for (func1, func2) in related {
            neighbours.entry(func1).and_modify(|x| x.push(func2));
            neighbours.entry(func2).and_modify(|x| x.push(func1));
        }

        self.0.observe(num_components(neighbours));
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "lcom4_per_impl_block",
        ImplLcom4::default(),
        |v| v.0,
        |v| Monoid::reduce(v.iter().cloned()),
    )
    .make_box()
}

fn num_components(neighbours: HashMap<&ImplItemFn, Vec<&ImplItemFn>>) -> usize {
    let mut to_visit: Vec<_> = neighbours.keys().cloned().collect();
    let mut out = 0;
    while !to_visit.is_empty() {
        out += 1;
        let next = to_visit.pop().expect("pop from a nonempty vec");
        walk(next, &neighbours, &mut to_visit);
    }
    out
}

fn walk(
    current: &ImplItemFn,
    neighbours: &HashMap<&ImplItemFn, Vec<&ImplItemFn>>,
    to_visit: &mut Vec<&ImplItemFn>,
) {
    let current_neigbours = neighbours.get(current).unwrap();
    let visit_next: Vec<_> = to_visit
        .iter()
        .cloned()
        .filter(|x| current_neigbours.contains(x))
        .collect();
    *to_visit = to_visit
        .iter()
        .filter(|x| !current_neigbours.contains(x))
        .copied()
        .collect();
    for next in visit_next {
        walk(next, neighbours, to_visit);
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use syn::parse_quote;

    use crate::collector::metrics::util::check;

    use super::ImplLcom4;

    #[test]
    fn cohesive() {
        let code = parse_quote! {
            impl Aboba {
                fn hehe(&self) {
                    self.a + self.b + self.aboba_call() // hehe uses a and b,
                                                        // method calls are not fields
                }
                fn haha(&mut self) {
                    self.b + self.c + self.aboba_call() // haha uses b and c
                }
            }
        };
        check::<ImplLcom4>(
            code,
            expect![[r#"
            {
              "sum": 1,
              "avg": 1.0,
              "mode": 1
            }"#]],
        );
    }

    #[test]
    fn not_cohesive() {
        let code = parse_quote! {
            impl Aboba {
                fn hehe(&self) {
                    self.a
                }
                fn haha(&mut self) {
                    self.b
                }
            }
        };
        check::<ImplLcom4>(
            code,
            expect![[r#"
            {
              "sum": 2,
              "avg": 2.0,
              "mode": 2
            }"#]],
        );
    }
}
