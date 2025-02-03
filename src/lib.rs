/// Contains the implementation of our camera that captures scenes from
/// the world
pub mod camera;

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

/// Contains the implementation of point lights, materials, and the
/// Phong reflection model to simulate the interaction of light with
/// objects
pub mod lights;

/// Contains representation of the world that contains lights
/// and objects
pub mod world;

/// Contains representations of the various shapes and implementations
/// for their common traits (such as surface normals, intersections, etc.)
pub mod shapes;

/// The `spatial` module contains the representation for key
/// three-dimensional spatial properties like Points and Vectors
pub mod spatial;
pub mod tick;
mod utils;
