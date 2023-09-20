mod visitor_collector;
pub use visitor_collector::VisitorCollector;

mod monoid;
pub use monoid::Monoid;

mod histogram;
pub use histogram::Hist;

pub fn merge_all<E: average::Merge + Default>(items: &[E]) -> E {
    let mut merged = E::default();
    for item in items {
        merged.merge(item);
    }
    merged
}
