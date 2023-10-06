use std::collections::{HashMap, HashSet};

use syn::{Ident, ImplItem, ImplItemFn};

use crate::collector::metrics::util::{Hist, Monoid};

use super::{
    prelude::*,
    util::{FnFieldSet, FnMethodCalls},
};

/// lack of cohesion of methods per impl block
#[derive(Default, Serialize)]
struct ImplLcom4(Hist<8>);

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
            .flat_map(|x| funcs.iter().map(move |y| (x, y)))
            .filter(|(x, y)| x != y);
        let related: Vec<(ImplItemFn, ImplItemFn)> = func_pairs
            .filter(|(func1, func2)| {
                let by_fields = fields.related(func1, func2);

                let by_calls = calls.related(func1, func2);
                by_fields || by_calls
            })
            .map(|(&x, &y)| (x.to_owned(), y.to_owned()))
            .collect();

        let mut neighbours: HashMap<ImplItemFn, Vec<ImplItemFn>> = HashMap::new();
        for func in funcs {
            neighbours.insert(func.clone(), vec![]);
        }
        for (func1, func2) in related {
            neighbours
                .entry(func1.clone())
                .and_modify(|x| x.push(func2.clone()));
            neighbours
                .entry(func2)
                .and_modify(|x| x.push(func1.clone()));
        }

        self.0.observe(num_components(&neighbours));
    }
}

pub fn make_collector() -> MetricCollectorBox {
    util::VisitorCollector::new(
        "lcom4_per_impl_block",
        ImplLcom4::default(),
        |v| v.0,
        |v| Monoid::reduce(v.into_iter().cloned()),
    )
    .make_box()
}

fn num_components(neighbours: &HashMap<ImplItemFn, Vec<ImplItemFn>>) -> usize {
    let mut to_visit: Vec<_> = neighbours.keys().collect();
    let mut out = 0;
    while !to_visit.is_empty() {
        out += 1;
        let next = to_visit.pop().expect("pop from a nonempty vec");
        walk(next, neighbours, &mut to_visit);
    }
    out
}

fn walk(
    current: &ImplItemFn,
    neighbours: &HashMap<ImplItemFn, Vec<ImplItemFn>>,
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
        .map(|&x| x)
        .collect();
    for next in visit_next {
        walk(next, neighbours, to_visit);
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, visit::Visit, File};

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
        let syntax_tree: File = syn::parse2(code).unwrap();
        let mut lcom4 = ImplLcom4::default();
        lcom4.visit_file(&syntax_tree);
        dbg!(lcom4.0.sum());
        assert_eq!(lcom4.0.sum(), 1);
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
        let syntax_tree: File = syn::parse2(code).unwrap();
        let mut lcom4 = ImplLcom4::default();
        lcom4.visit_file(&syntax_tree);
        assert_eq!(lcom4.0.sum(), 2);
    }
}
