mod visitor_collector;

pub use visitor_collector::VisitorCollector;

pub fn merge_all<E: average::Merge + Default>(items: &[E]) -> E {
    let mut merged = E::default();
    for item in items {
        merged.merge(item);
    }
    merged
}
