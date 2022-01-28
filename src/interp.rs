use crate::ndarr::VecNDArray;
use crate::traits::{Idx, UnboundedIndexable};
use interpolation::Lerp;
use std::marker::PhantomData;

pub enum Interpolation {
    Linear,
    Quadratic,
    Cubic,
}

impl Interpolation {
    fn factors(&self, loc: f64) -> (Vec<isize>, f64) {
        match self {
            Self::Linear => {
                let floor = loc.floor();
                let factor = loc - floor;
                let ceil = (loc + 1.0) as isize;
                (vec![floor as isize, ceil], factor)
            },
            Self::Quadratic => {
                let mid = loc.round();
                let factor = (1.0 + loc - mid) / 2.0;
                let floor = (mid - 1.0) as isize;
                let v = vec![floor, floor + 1, floor + 2];
                (v, factor)
            },
            Self::Cubic => {
                let closest = loc.round();
                let factor = (1.0 + loc - closest) / 3.0;
                let iclosest = closest as isize;
                if closest <= loc {
                    (vec![iclosest - 1, iclosest, iclosest + 1, iclosest + 2], factor)
                } else {
                    (vec![iclosest - 2, iclosest - 1, iclosest, iclosest + 1], factor)
                }
            },
        }
    }
}

pub struct InterpWrapper<T, I1: Idx, U: UnboundedIndexable<T, I1, D>, const D: usize> {
    data: U,
    interpolation: Option<Interpolation>,
    _t: PhantomData<T>,
    _i1: PhantomData<I1>,
}

impl<T, U, const D: usize> UnboundedIndexable<T, f64, D> for InterpWrapper<T, isize, U, D>
where
    T: Lerp,
    U: UnboundedIndexable<T, isize, D>,
{
    fn get(&self, idx: &[f64; D]) -> &T {
        if let Some(int) = &self.interpolation {

            match int {
                Interpolation::Linear => {
                    unimplemented!()
                }
                Interpolation::Quadratic => {
                    unimplemented!()
                }
                Interpolation::Cubic => {
                    unimplemented!()
                }
            }
        } else {
            self.data.get(&idx.map(|v| v.round() as isize))
        }
    }
}
