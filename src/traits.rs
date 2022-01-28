use num_traits::Zero;
use std::fmt::Debug;

pub trait Idx: Debug + Copy + Zero {}
impl<T> Idx for T where T: Debug + Copy + Zero {}

pub trait BoundedIndexable<T, I: Idx, const D: usize> {
    fn get(&self, idx: &[I; D]) -> Option<&T>;

    fn bounds(&self) -> [(I, I); D];

    fn multiget(&self, idxs: &[[I; D]]) -> Vec<Option<&T>> {
        idxs.iter().map(|idx| self.get(idx)).collect()
    }
}

pub trait UnboundedIndexable<T, I: Idx, const D: usize> {
    fn get(&self, idx: &[I; D]) -> &T;

    fn multiget(&self, idxs: &[[I; D]]) -> Vec<&T> {
        idxs.iter().map(|idx| self.get(idx)).collect()
    }
}
