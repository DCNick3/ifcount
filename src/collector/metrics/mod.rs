mod r#impl;
pub mod util;

use super::FileAst;
use serde::Serialize;
use tracing::info_span;

/// A type-erased metric collector
pub struct MetricCollectorBox(Box<dyn MetricCollectorBoxed>);

impl MetricCollectorBox {
    pub fn name(&self) -> &'static str {
        self.0.name()
    }

    pub fn collect_metric(&self, files: &[FileAst]) -> serde_json::Value {
        self.0.collect_metric(files)
    }

    pub fn make_box(self) -> Self {
        self
    }
}

pub trait MetricCollector: Sized + 'static {
    type Metric;
    type AggregatedMetric: Serialize;

    fn name(&self) -> &'static str;

    fn collect_file(&self, file: &FileAst) -> Self::Metric;

    fn aggregate_metrics(&self, metric: &[Self::Metric]) -> Self::AggregatedMetric;

    fn make_box(self) -> MetricCollectorBox {
        MetricCollectorBox(Box::new(self))
    }
}

// An object-safe wrapper for MetricCollector
trait MetricCollectorBoxed {
    fn name(&self) -> &'static str;

    fn collect_metric(&self, files: &[FileAst]) -> serde_json::Value;
}

impl<M: Serialize, C: MetricCollector<AggregatedMetric = M>> MetricCollectorBoxed for C {
    fn name(&self) -> &'static str {
        C::name(self)
    }

    #[tracing::instrument(skip(self, files), fields(metric = %self.name()))]
    fn collect_metric(&self, files: &[FileAst]) -> serde_json::Value {
        // TODO: parallel?
        let metrics = files
            .iter()
            .map(|file| {
                let _span = info_span!("collect_file", file = %file.path).entered();
                self.collect_file(file)
            })
            .collect::<Vec<_>>();
        let metric = self.aggregate_metrics(&metrics);
        serde_json::to_value(metric).expect("Metric should be serializable")
    }
}

pub use r#impl::get_metric_collectors;
