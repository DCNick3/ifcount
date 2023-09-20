pub trait Monoid: Sized {
    fn init() -> Self;
    fn unite(self, rhs: Self) -> Self;
    fn reduce(iter: impl Iterator<Item = Self>) -> Self {
        iter.fold(Self::init(), |x, y| x.unite(y))
    }
}
