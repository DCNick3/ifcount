use crate::collector::metrics::MetricCollector;
use crate::collector::FileAst;
use serde::Serialize;
use std::marker::PhantomData;

/// A helper for collecting metrics using a syn visitor
pub struct VisitorCollector<
    V: for<'ast> syn::visit::Visit<'ast> + Default + 'static,
    M: 'static,
    AM: Serialize + 'static,
    Extract: Fn(V) -> M + 'static,
    Aggregate: Fn(&[M]) -> AM + 'static,
> {
    name: &'static str,
    extract: Extract,
    aggregate: Aggregate,
    phantom: PhantomData<(V, Extract, Aggregate)>,
}

impl<
        V: for<'ast> syn::visit::Visit<'ast> + Default + 'static,
        M: 'static,
        AM: Serialize + 'static,
        Extract: Fn(V) -> M + 'static,
        Aggregate: Fn(&[M]) -> AM + 'static,
    > VisitorCollector<V, M, AM, Extract, Aggregate>
{
    /// Create a new visitor collector
    ///
    /// You need to supply: a metric name, a visitor (used only for getting a type), a function to extract a metric from a visitor (that was run on a file), and a function to aggregate metrics across files.
    pub fn new(name: &'static str, _visitor: V, extract: Extract, aggregate: Aggregate) -> Self {
        Self {
            name,
            extract,
            aggregate,
            phantom: PhantomData,
        }
    }
}

impl<
        V: for<'ast> syn::visit::Visit<'ast> + Default + 'static,
        M: 'static,
        AM: Serialize + 'static,
        Extract: Fn(V) -> M + 'static,
        Aggregate: Fn(&[M]) -> AM + 'static,
    > MetricCollector for VisitorCollector<V, M, AM, Extract, Aggregate>
{
    type Metric = M;
    type AggregatedMetric = AM;

    fn name(&self) -> &'static str {
        self.name
    }

    fn collect_file(&self, file: &FileAst) -> Self::Metric {
        let mut visitor = V::default();
        visitor.visit_file(&file.content);
        (self.extract)(visitor)
    }

    fn aggregate_metrics(&self, metric: &[Self::Metric]) -> Self::AggregatedMetric {
        (self.aggregate)(metric)
    }
}
