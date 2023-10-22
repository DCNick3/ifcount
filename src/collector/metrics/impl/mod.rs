use prelude::util::{Hist, Unaggregated};

use super::MetricCollectorBox;

mod prelude {
    pub use crate::collector::metrics::{util, MetricCollector, MetricCollectorBox};
    pub use serde::Serialize;
    pub use syn::visit::Visit;
}

macro_rules! collectors {
    (
        $($collector:expr),*
        $(,)?
    ) => {
        vec![$($collector.make_box()),*]
    };
}

mod basic_enums;
mod basic_files;
mod basic_structs;
mod basic_traits;
mod complexity;
mod fn_arg_count;
mod fn_depth;
mod if_count;
mod macros;
mod methods;
mod stmt_size;

pub fn get_metric_collectors() -> Vec<MetricCollectorBox> {
    collectors![
        fn_depth::make_collector(),
        if_count::make_collector(),
        fn_arg_count::make_collector::<Unaggregated>(),
        basic_structs::make_collector::<Unaggregated>(),
        basic_enums::make_collector::<Unaggregated>(),
        basic_traits::make_collector::<Unaggregated>(),
        complexity::make_collector::<Unaggregated>(),
        stmt_size::make_collector::<Unaggregated>(),
        basic_files::make_collector::<Unaggregated>(),
        methods::make_collector::<Unaggregated>(),
        macros::make_collector(),
    ]
}
