use serde::Serialize;

use super::{super::Monoid, Observer};

#[derive(Default, Clone)]
pub struct Unaggregated<T = usize> {
    observations: Vec<T>,
}

impl<T> std::ops::Add for Unaggregated<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.unite(rhs)
    }
}
impl<T: Serialize> Serialize for Unaggregated<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.observations.serialize(serializer)
    }
}

impl<T> Monoid for Unaggregated<T> {
    fn init() -> Self {
        Self {
            observations: Vec::new(),
        }
    }

    fn unite(self, mut rhs: Self) -> Self {
        let mut combined = self.observations;
        combined.append(&mut rhs.observations);
        Self {
            observations: combined,
        }
    }
}

impl<T> Observer<T> for Unaggregated<T> {
    fn observe(&mut self, value: T) {
        self.observations.push(value);
    }

    fn count(&self) -> usize {
        self.observations.len()
    }
}
