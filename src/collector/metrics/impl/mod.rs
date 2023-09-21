use syn::visit::Visit;

use super::MetricCollectorBox;

mod prelude {
    pub use super::Visitor;
    pub use crate::collector::{
        metrics::{util, MetricCollector, MetricCollectorBox},
        FileAst,
    };
    pub use serde::Serialize;
    pub use syn::visit::Visit;
}

macro_rules! collectors {
    ($($collector:expr),*) => {
        vec![$($collector.make_box()),*]
    };
}

pub trait Visitor: for<'ast> Visit<'ast> {
    fn visitor() -> MetricCollectorBox;
}

mod avg_fn_arg_count;
mod avg_fn_depth;
mod basic_enums;
mod basic_structs;
mod basic_traits;
mod if_count;

pub fn get_metric_collectors() -> Vec<MetricCollectorBox> {
    collectors![
        avg_fn_depth::visitor(),
        if_count::visitor(),
        avg_fn_arg_count::FnArgsHist::visitor(),
        basic_structs::Structs::visitor(),
        basic_enums::Enums::visitor(),
        basic_traits::TraitDefinitions::visitor()
    ]
}
