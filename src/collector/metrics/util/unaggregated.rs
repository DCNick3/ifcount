use serde::Serialize;

use super::Monoid;

#[derive(Default, Clone)]
pub struct Unaggregated {
    observations: Vec<usize>,
}

impl std::ops::Add for Unaggregated {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.unite(rhs)
    }
}
impl Serialize for Unaggregated {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.observations.serialize(serializer)
    }
}

impl Monoid for Unaggregated {
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

impl Unaggregated {
    pub fn observe(&mut self, val: usize) {
        self.observations.push(val);
    }
}
