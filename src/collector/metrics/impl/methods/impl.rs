use std::collections::HashMap;

use graph::prelude::UndirectedCsrGraph;
use syn::{Ident, ImplItem, ImplItemFn};

use crate::collector::metrics::util::Hist;

use super::{
    prelude::*,
    util::{FnFieldSet, FnMethodCalls},
};

/// lack of cohesion of methods per impl block
struct ImplCohesion(Hist<8>);

impl Visit<'_> for ImplCohesion {
    fn visit_item_impl(&mut self, i: &'_ syn::ItemImpl) {
        let funcs: Vec<_> = i
            .items
            .iter()
            .flat_map(|x| match x {
                ImplItem::Fn(func) => Some(func),
                _ => None,
            })
            .collect();
        let mut fields = FnFieldSet::default();
        let mut calls = FnMethodCalls::default();
        for func in &funcs {
            fields.visit_impl_item_fn(func);
            calls.visit_impl_item_fn(func);
        }
        let related: HashMap<ImplItemFn, Ident> = HashMap::new();
        let func_pairs = funcs
            .iter()
            .flat_map(|x| funcs.iter().map(move |y| (x, y)))
            .filter(|(x, y)| x != y);
        let related: Vec<_> = func_pairs
            .filter(|(func1, func2)| {
                let by_fields = fields.related(func1, func2);

                let by_calls = calls.related(func1, func2);
                by_fields || by_calls
            })
            .map(|(&x, &y)| (x.to_owned(), y.to_owned()))
            .collect();

        let graph: UndirectedCsrGraph<ImplItemFn> =
            graph::prelude::GraphBuilder::new().edges(related).build();
    }
}

fn num_components(related_map: &HashMap<ImplItemFn, ImplItemFn>) -> usize {
    let mut visited = HashMap::new();
}
