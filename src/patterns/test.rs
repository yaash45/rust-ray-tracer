use super::Pattern;
use crate::{
    color::Color,
    matrix::{Matrix, Transformable},
    spatial::Tuple,
};

/// A pattern that is used only for testing
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TestPattern {
    transform_matrix: Matrix<4, 4>,
}

impl TestPattern {
    /// Create a new test pattern that does not have any colors,
    /// but just a transformation matrix
    pub fn new() -> Self {
        Self::default()
    }
}

impl Transformable for TestPattern {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix
    }
}

impl Default for TestPattern {
    fn default() -> Self {
        Self {
            transform_matrix: Matrix::<4, 4>::identity(),
        }
    }
}

impl Pattern for TestPattern {
    /// Returns a Color based on the x, y, and z coordinates of the given point.
    /// This function is intended for testing purposes and simply constructs a
    /// color where each component corresponds to the respective coordinate value
    /// of the input Tuple.
    ///
    /// This way, we can check if the `pattern_at` receives the correctly transformed
    /// point values.
    fn pattern_at(&self, point: &Tuple) -> Color {
        Color::new(point.get_x(), point.get_y(), point.get_z())
    }
}
