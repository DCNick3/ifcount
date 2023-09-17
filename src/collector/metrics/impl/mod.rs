use super::MetricCollectorBox;

mod prelude {
    pub use crate::collector::{
        metrics::{util, MetricCollector, MetricCollectorBox},
        FileAst,
    };
    pub use syn::visit::Visit;
}

macro_rules! collectors {
    ($($collector:expr),*) => {
        vec![$($collector.make_box()),*]
    };
}

mod avg_fn_depth;
mod if_count;

pub fn get_metric_collectors() -> Vec<MetricCollectorBox> {
    collectors![avg_fn_depth::visitor(), if_count::visitor()]
}
