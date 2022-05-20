//! Basic traits.
use num_traits::Zero;
use std::fmt::Debug;

/// Value used for indexing a single dimension of an array.
/// N-dimensional arrays are indexed using an N-length array of these.
pub trait Idx: Debug + Copy + Zero {}
impl<T> Idx for T where T: Debug + Copy + Zero {}

/// Trait for indexing into some discrete, finite data.
pub trait BoundedIndexable<T, I: Idx, const D: usize> {
    /// Get a single value from the data, or None if the index is out of bounds.
    fn get(&self, idx: &[I; D]) -> Option<&T>;

    /// Get (min, max) indices of the data in each dimension.
    fn bounds(&self) -> [(I, I); D];

    /// Get multiple values at once.
    fn multiget(&self, idxs: &[[I; D]]) -> Vec<Option<&T>> {
        idxs.iter().map(|idx| self.get(idx)).collect()
    }
}

/// Trait for indexing into discrete, but infinite data.
pub trait UnboundedIndexable<T, I: Idx, const D: usize> {
    /// Get a single value from the data, which always exists.
    fn get(&self, idx: &[I; D]) -> &T;

    /// Get multiple values at once.
    fn multiget(&self, idxs: &[[I; D]]) -> Vec<&T> {
        idxs.iter().map(|idx| self.get(idx)).collect()
    }
}
