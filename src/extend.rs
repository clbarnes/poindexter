//! Utilities for extending an array's domain with the same range.
use crate::adapt::{Adapter, Adapter1D, AdapterND};

#[derive(Copy, Clone, Debug)]
pub enum ExtenderType {
    /// Find the nearest existing point.
    Nearest,
    /// max_index + 1 = min_index
    Wrap,
    /// max_index + 1 = max_index; max_index + 2 = max_index - 1, etc.
    Reflect,
}

/// Adapter1D which snaps the given dimension coordinate to the real dimension coordinate where its data should be found.
#[derive(Copy, Clone, Debug)]
pub struct Extender1D {
    length: usize,
    e_type: ExtenderType,
}

impl Extender1D {
    pub fn new(length: usize, e_type: ExtenderType) -> Self {
        Self { length, e_type }
    }
}

impl Adapter1D<isize, usize> for Extender1D {
    fn adapt(&self, idx: isize) -> usize {
        match self.e_type {
            ExtenderType::Nearest => {
                if idx < 0 {
                    0
                } else if idx as usize >= self.length {
                    self.length - 1
                } else {
                    idx as usize
                }
            }
            ExtenderType::Wrap => {
                let abs_i = idx.abs() as usize;
                let rem = abs_i % self.length;
                if idx > 0 || rem == 0 {
                    rem
                } else {
                    self.length - rem
                }
            }
            ExtenderType::Reflect => {
                let abs_i = idx.abs() as usize;
                let div = abs_i / self.length;
                let mut rem = abs_i % self.length;
                if idx < 0 {
                    if rem == 0 {
                        rem = self.length - 1;
                    } else {
                        rem -= 1;
                    }
                }
                if div % 2 == 0 {
                    rem
                } else {
                    self.length - rem - 1
                }
            }
        }
    }
}

/// Adapter to map a whole coordinate to a coordinate where the data actually exists.
pub struct Extender<const D: usize> {
    adapter: AdapterND<isize, usize, Extender1D, D>,
}

impl<const D: usize> Extender<D> {
    pub fn new(shape: &[usize; D], e_type: ExtenderType) -> Self {
        Self {
            adapter: AdapterND::new(shape.map(|s| Extender1D::new(s, e_type))),
        }
    }
}

impl<const D: usize> Adapter<isize, usize, D> for Extender<D> {
    fn adapt(&self, idx: &[isize; D]) -> [usize; D] {
        self.adapter.adapt(idx)
    }
}
