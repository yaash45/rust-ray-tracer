/// Contains the main canvas that we represent our coloured pixels on.
/// It also contains the functionality to export the canvas into a
/// familiar image format (PPM)
pub mod canvas;

/// This module contains our representation of RGB Color values
/// and implementations of various color operations
pub mod color;

/// Contains the representation for matrices
/// and methods to operate on them
pub mod matrix;

/// Contains the implementation for our Rays and their intersections
/// with various types of objects
pub mod intersections;

pub mod lights;

/// The `spatial` module contains the representation for key
/// three-dimensional spatial properties like Points and Vectors
pub mod spatial;
pub mod tick;
mod utils;
