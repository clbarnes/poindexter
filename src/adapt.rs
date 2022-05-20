//! Adapt coordinates in one space into another space.
use crate::traits::Idx;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

/// Something which is able to adapt a single dimension of a coordinate.
pub trait Adapter1D<In: Idx, Out: Idx> {
    /// Adapt a single dimension of a coordinate into another.
    fn adapt(&self, idx: In) -> Out;
}

/// Something which is able to adapt a whole coordinate.
pub trait Adapter<In: Idx, Out: Idx, const D: usize> {
    /// Adapt one coordinate into another.
    fn adapt(&self, idx: &[In; D]) -> [Out; D];
}

/// An Adapter for N-D coordinates comprised of N Adapter1Ds.
pub struct AdapterND<In: Idx, Out: Idx, Ad: Adapter1D<In, Out>, const D: usize> {
    pub adapters: [Ad; D],
    _in: PhantomData<In>,
    _out: PhantomData<Out>,
}

impl<In: Idx, Out: Idx, Ad: Adapter1D<In, Out>, const D: usize> AdapterND<In, Out, Ad, D> {
    pub fn new(adapters: [Ad; D]) -> Self {
        AdapterND {
            adapters,
            _in: PhantomData {},
            _out: PhantomData {},
        }
    }
}

/// Utility applying a function to an index array by mapping a function across it and some other argument array.
fn idx_zip<In: Idx, Out: Idx, T, F, const D: usize>(
    idx: &[In; D],
    arr2: &[T; D],
    func: F,
) -> [Out; D]
where
    F: FnMut((&In, &T)) -> Out,
{
    let mut out = [Out::zero(); D];
    for (i, val) in idx.iter().zip(arr2.iter()).map(func).enumerate() {
        out[i] = val;
    }
    out
}

impl<In: Idx, Out: Idx, Ad: Adapter1D<In, Out>, const D: usize> Adapter<In, Out, D>
    for AdapterND<In, Out, Ad, D>
{
    fn adapt(&self, idx: &[In; D]) -> [Out; D] {
        idx_zip(idx, &self.adapters, |(i, ad)| ad.adapt(*i))
    }
}

/// Adapter which scales a coordinate:
/// the input coordinates are multiplied by the scale factors to produce the output coordinates.
#[derive(Copy, Clone, Debug)]
pub struct Scale<I: Idx + Mul, const D: usize> {
    factors: [I; D],
}

impl<I: Idx + Mul<Output = I>, const D: usize> Adapter<I, I, D> for Scale<I, D> {
    fn adapt(&self, idx: &[I; D]) -> [I; D] {
        idx_zip(idx, &self.factors, |(i, f)| *i * *f)
    }
}

/// Adapter with offsets/ translates a coordinate:
/// the factor is added to the input coordinate.
#[derive(Copy, Clone, Debug)]
pub struct Offset<I: Idx + Add, const D: usize> {
    factors: [I; D],
}

impl<I: Idx + Add<Output = I>, const D: usize> Adapter<I, I, D> for Offset<I, D> {
    fn adapt(&self, idx: &[I; D]) -> [I; D] {
        idx_zip(idx, &self.factors, |(i, f)| *i + *f)
    }
}
