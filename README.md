# poindexter

Experiments with extended ND array access in rust.

Inspired by ImageJ.

## Features

- `extend` an array by reflection, wrapping, constant value, or nearest-value (zero memory cost)
- `adapt` coordinates from one space to another by scaling, offset etc.
- `interp`olate values from a discrete ND array

## Example use case

You have a 3D array of floats which represent samples on a regular grid, and are accessed by a `[usize; 3]` index.
This is a `B: BoundedIndexable<f64, usize, 3>`.

We might try to access outside of this area (including in the negatives), where the value should be a 0: so, it is wrapped in a `U: UnboundedIndexableWrapperConstant<f64, isize, B, 3>`.

You want to be able to access the nearest values of a continuous (float) coordinate, rather than exact pixel indices, so you wrap it in a `R: RealIndexableWrapperNearest<f64, f64, U, 3>`.

Then, you want to adapt your coordinates so that you can address real-world locations and have them redirected to places in the array, using a `A: AdaptingWrapper<f64, f64, f64, Adapter<f64, f64, 3>, R, D>`.
