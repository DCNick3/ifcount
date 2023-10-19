mod visitor_collector;
pub use visitor_collector::VisitorCollector;

mod monoid;
pub use monoid::Monoid;

mod histogram;
pub use histogram::Hist;

mod test;
pub use test::check;
