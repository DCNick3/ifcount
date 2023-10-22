// polymorphic over type of observations
pub trait Observer<T = usize> {
    fn observe(&mut self, value: T);
}

mod histogram;
mod unaggregated;

pub use histogram::Hist;
pub use unaggregated::Unaggregated;
