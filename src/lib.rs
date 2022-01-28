use std::marker::PhantomData;

pub mod traits;
use crate::traits::{BoundedIndexable, Idx, UnboundedIndexable};
pub mod adapt;
use crate::adapt::Adapter;
pub mod extend;
use crate::extend::Extender;
pub mod interp;

pub mod ndarr;

pub struct UnboundedIndexableWrapperConstant<
    T,
    I1: Idx,
    B: BoundedIndexable<T, I1, D>,
    const D: usize,
> {
    data: B,
    cval: T,
    _i1: PhantomData<I1>,
}

impl<T, I1: Idx, B: BoundedIndexable<T, I1, D>, const D: usize>
    UnboundedIndexableWrapperConstant<T, I1, B, D>
{
    pub fn new(wrapped: B, cval: T) -> Self {
        Self {
            data: wrapped,
            cval,
            _i1: PhantomData,
        }
    }
}

impl<T, B: BoundedIndexable<T, usize, D>, const D: usize> UnboundedIndexable<T, isize, D>
    for UnboundedIndexableWrapperConstant<T, usize, B, D>
{
    fn get(&self, idx: &[isize; D]) -> &T {
        let mut u_idx: [usize; D] = [0; D];
        for (i, val) in idx.iter().enumerate() {
            if val < &0 {
                return &self.cval;
            }
            u_idx[i] = *val as usize;
        }
        match self.data.get(&u_idx) {
            Some(c) => c,
            _ => &self.cval,
        }
    }
}

pub struct UnboundedIndexableWrapperExtend<
    T,
    I1: Idx,
    B: BoundedIndexable<T, I1, D>,
    const D: usize,
> {
    data: B,
    extender: Extender<D>,
    _t: PhantomData<T>,
    _i1: PhantomData<I1>,
}

impl<T, I1: Idx, B: BoundedIndexable<T, I1, D>, const D: usize>
    UnboundedIndexableWrapperExtend<T, I1, B, D>
{
    pub fn new(wrapped: B, extender: Extender<D>) -> Self {
        Self {
            data: wrapped,
            extender,
            _t: PhantomData {},
            _i1: PhantomData {},
        }
    }
}

impl<T, B: BoundedIndexable<T, usize, D>, const D: usize> UnboundedIndexable<T, isize, D>
    for UnboundedIndexableWrapperExtend<T, usize, B, D>
{
    fn get(&self, idx: &[isize; D]) -> &T {
        let adapted = self.extender.adapt(idx);
        println!("adapted {:?}", adapted);
        self.data.get(&adapted).unwrap()
    }
}

pub struct RealIndexableWrapperNearest<T, I1: Idx, U: UnboundedIndexable<T, I1, D>, const D: usize>
{
    data: U,
    _t: PhantomData<T>,
    _i1: PhantomData<I1>,
}

impl<T, I1: Idx, U: UnboundedIndexable<T, I1, D>, const D: usize>
    RealIndexableWrapperNearest<T, I1, U, D>
{
    pub fn new(wrapped: U) -> Self {
        Self {
            data: wrapped,
            _t: PhantomData,
            _i1: PhantomData,
        }
    }
}

impl<T, U: UnboundedIndexable<T, isize, D>, const D: usize> UnboundedIndexable<T, f64, D>
    for RealIndexableWrapperNearest<T, isize, U, D>
{
    fn get(&self, idx: &[f64; D]) -> &T {
        self.data.get(&idx.map(|val| val.round() as isize))
    }
}

pub struct AdaptingWrapper<T, I1, I2, U, A, const D: usize>
where
    I1: Idx,
    I2: Idx,
    U: UnboundedIndexable<T, I2, D>,
    A: Adapter<I1, I2, D>,
{
    data: U,
    adapter: A,
    _t: PhantomData<T>,
    _i1: PhantomData<I1>,
    _i2: PhantomData<I2>,
}

impl<T, I1, I2, A, U, const D: usize> UnboundedIndexable<T, I1, D>
    for AdaptingWrapper<T, I1, I2, U, A, D>
where
    I1: Idx,
    I2: Idx,
    A: Adapter<I1, I2, D>,
    U: UnboundedIndexable<T, I2, D>,
{
    fn get(&self, idx: &[I1; D]) -> &T {
        let adapted = self.adapter.adapt(idx);
        self.data.get(&adapted)
    }
}

#[cfg(test)]
mod tests {
    use super::extend::ExtenderType;
    use super::*;

    pub struct BoundedIndexableVec<T> {
        data: Vec<T>,
    }

    impl<T> BoundedIndexable<T, usize, 1> for BoundedIndexableVec<T> {
        fn get(&self, idx: &[usize; 1]) -> Option<&T> {
            self.data.get(idx[0])
        }

        fn bounds(&self) -> [(usize, usize); 1] {
            [(0, self.data.len())]
        }
    }

    #[test]
    fn regular_vec() {
        let v = vec![1, 2, 3, 4];
        assert!(v.get(0) == Some(&1));
        assert!(v.get(4) == None);
    }

    #[test]
    fn wrapped_vec() {
        let b = BoundedIndexableVec {
            data: vec![1, 2, 3, 4],
        };
        assert!(b.get(&[0; 1]) == Some(&1));
        assert!(b.get(&[4; 1]) == None);
    }

    #[test]
    fn unbounded_vec_const() {
        let b = BoundedIndexableVec {
            data: vec![0, 1, 2, 3],
        };
        let u = UnboundedIndexableWrapperConstant::new(b, 10);
        assert_eq!(u.get(&[0; 1]), &0);
        assert_eq!(u.get(&[4; 1]), &10);
        assert_eq!(u.get(&[-1; 1]), &10);
        u.get(&[1_000_000; 1]);
        u.get(&[-1_000_000; 1]);
    }

    #[test]
    fn unbounded_vec_nearest() {
        let b = BoundedIndexableVec {
            data: vec![0, 1, 2, 3],
        };
        let len = b.data.len();
        let u =
            UnboundedIndexableWrapperExtend::new(b, Extender::new(&[len], ExtenderType::Nearest));
        assert_eq!(u.get(&[0; 1]), &0);
        assert_eq!(u.get(&[4; 1]), &3);
        assert_eq!(u.get(&[-1; 1]), &0);
        u.get(&[1_000_000; 1]);
        u.get(&[-1_000_000; 1]);
    }

    #[test]
    fn unbounded_vec_wrap() {
        let b = BoundedIndexableVec {
            data: vec![0, 1, 2, 3],
        };
        let len = b.data.len();
        let u = UnboundedIndexableWrapperExtend::new(b, Extender::new(&[len], ExtenderType::Wrap));
        assert_eq!(u.get(&[0; 1]), &0);
        assert_eq!(u.get(&[4; 1]), &0);
        assert_eq!(u.get(&[-1; 1]), &3);
        u.get(&[1_000_000; 1]);
        u.get(&[-1_000_000; 1]);
    }

    #[test]
    fn unbounded_vec_reflect() {
        let b = BoundedIndexableVec {
            data: vec![0, 1, 2, 3],
        };
        let len = b.data.len();
        let u =
            UnboundedIndexableWrapperExtend::new(b, Extender::new(&[len], ExtenderType::Reflect));
        assert_eq!(u.get(&[0; 1]), &0);
        assert_eq!(u.get(&[4; 1]), &3);
        assert_eq!(u.get(&[8; 1]), &0);
        assert_eq!(u.get(&[-1; 1]), &0);
        assert_eq!(u.get(&[-5; 1]), &3);
        u.get(&[1_000_000; 1]);
        u.get(&[-1_000_000; 1]);
    }

    #[test]
    fn real_vec() {
        let b = BoundedIndexableVec {
            data: vec![0, 1, 2, 3],
        };
        let u = UnboundedIndexableWrapperConstant::new(b, 10);
        let r = RealIndexableWrapperNearest::new(u);
        assert_eq!(r.get(&[0.0; 1]), &0);
        assert_eq!(r.get(&[2.4; 1]), &2);
        assert_eq!(r.get(&[-1.0; 1]), &10);
    }
}
