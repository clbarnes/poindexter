//! Simple implementation of an N-dimensional array, mainly for testing.
use crate::traits::BoundedIndexable;

pub enum Order {
    RowMajor,
    ColumnMajor,
}

impl Default for Order {
    fn default() -> Self {
        Self::RowMajor
    }
}

impl Order {
    /// Convert an array of dimensional indices into a linear index; None if out of bounds.
    pub fn index<const D: usize>(&self, shape: &[usize; D], index: &[usize; D]) -> Option<usize> {
        let mut total = 0;
        match self {
            Self::RowMajor => {
                let mut prev_s = 1;
                for (s, i) in shape.iter().rev().zip(index.iter().rev()) {
                    if i >= s {
                        return None;
                    }
                    total += i * prev_s;
                    prev_s = *s;
                }
                Some(total)
            }
            Self::ColumnMajor => {
                let mut prev_s = 1;
                for (s, i) in shape.iter().zip(index.iter()) {
                    if i >= s {
                        return None;
                    }
                    total += i * prev_s;
                    prev_s = *s;
                }
                Some(total)
            }
        }
    }
}

/// ND array backed by a Vec.
pub struct VecNDArray<T, const D: usize> {
    data: Vec<T>,
    shape: [usize; D],
    order: Order,
}

impl<T, const D: usize> VecNDArray<T, D> {
    /// Checks that all axes have nonzero length, and that the given shape matches the data length.
    pub fn new(data: Vec<T>, shape: [usize; D], order: Order) -> Result<Self, &'static str> {
        let mut prod = 1;
        for s in shape {
            if s == 0 {
                return Err("Zero-dimensional axis");
            }
            prod *= s;
        }
        if data.len() != prod {
            return Err("Shape does not match underlying data");
        }
        Ok(Self::new_unchecked(data, shape, order))
    }

    /// Skips checks.
    pub fn new_unchecked(data: Vec<T>, shape: [usize; D], order: Order) -> Self {
        Self { data, shape, order }
    }
}

impl<T, const D: usize> BoundedIndexable<T, usize, D> for VecNDArray<T, D> {
    fn get(&self, idx: &[usize; D]) -> Option<&T> {
        self.order
            .index(&self.shape, idx)
            .map(|lin| &self.data[lin])
    }

    fn bounds(&self) -> [(usize, usize); D] {
        self.shape.map(|s| (0, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::BoundedIndexable;

    #[test]
    fn order_rowmaj() {
        let order = Order::RowMajor;
        let shape = [5, 3];
        assert_eq!(order.index(&shape, &[0, 0]).unwrap(), 0);
        assert_eq!(order.index(&shape, &[0, 1]).unwrap(), 1);
        assert_eq!(order.index(&shape, &[1, 0]).unwrap(), 3);
        assert_eq!(order.index(&shape, &[10, 10]), None);
    }

    #[test]
    fn order_colmaj() {
        let order = Order::ColumnMajor;
        let shape = [5, 3];
        assert_eq!(order.index(&shape, &[0, 0]).unwrap(), 0);
        assert_eq!(order.index(&shape, &[0, 1]).unwrap(), 5);
        assert_eq!(order.index(&shape, &[1, 0]).unwrap(), 1);
        assert_eq!(order.index(&shape, &[10, 10]), None);
    }

    #[test]
    fn nd_rowmaj() {
        let order = Order::RowMajor;
        let shape = [5, 3];
        let numel = shape.iter().product();
        let data: Vec<usize> = (0..numel).map(|v| v * 10).collect();
        let arr = VecNDArray::new(data, shape, order).unwrap();

        assert_eq!(*arr.get(&[0, 0]).unwrap(), 0);
        assert_eq!(*arr.get(&[0, 1]).unwrap(), 10);
        assert_eq!(*arr.get(&[1, 0]).unwrap(), 30);
        assert_eq!(arr.get(&[10, 10]), None);
    }

    #[test]
    fn nd_colmaj() {
        let order = Order::ColumnMajor;
        let shape = [5, 3];
        let numel = shape.iter().product();
        let data: Vec<usize> = (0..numel).map(|v| v * 10).collect();
        let arr = VecNDArray::new(data, shape, order).unwrap();

        assert_eq!(*arr.get(&[0, 0]).unwrap(), 0);
        assert_eq!(*arr.get(&[0, 1]).unwrap(), 50);
        assert_eq!(*arr.get(&[1, 0]).unwrap(), 10);
        assert_eq!(arr.get(&[10, 10]), None);
    }
}
