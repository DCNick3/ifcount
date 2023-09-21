use super::MetricCollectorBox;

mod prelude {
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

mod avg_fn_arg_count;
mod avg_fn_depth;
mod basic_enums;
mod basic_structs;
mod basic_traits;
mod complexity;
mod if_count;

pub fn get_metric_collectors() -> Vec<MetricCollectorBox> {
    collectors![
        avg_fn_depth::make_collector(),
        if_count::make_collector(),
        avg_fn_arg_count::make_collector(),
        basic_structs::make_collector(),
        basic_enums::make_collector(),
        basic_traits::make_collector(),
        complexity::make_collector()
    ]
}
