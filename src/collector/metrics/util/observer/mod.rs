// polymorphic over type of observations
pub trait Observer<T = usize> {
    fn observe(&mut self, value: T);
    fn count(&self) -> usize;
}

mod histogram;
mod unaggregated;

pub use histogram::Hist;
pub use unaggregated::Unaggregated;
