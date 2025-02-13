mod float_equals;

pub use float_equals::{float_equals, EPSILON};

#[cfg(test)]
pub mod test_utils {
    use crate::{
        color::Color,
        matrix::{Matrix, Transformable},
        patterns::Pattern,
        shapes::{Shape, Sphere},
        spatial::Tuple,
    };

    /// A factory for generating test shapes.
    pub struct TestShapeFactory {}

    /// Generates a default shape for testing.
    impl TestShapeFactory {
        pub fn test_shape() -> Shape {
            Shape::Sphere(Sphere::default())
        }
    }

    /// A factory for generating test patterns.
    pub struct PatternFactory {}

    impl PatternFactory {
        /// Generates a default pattern for testing.
        pub fn test_pattern() -> impl Pattern {
            TestPattern::new()
        }
    }

    /// A pattern that is used only for testing
    pub struct TestPattern {
        transform_matrix: Matrix<4, 4>,
    }

    impl TestPattern {
        /// Create a new test pattern that does not have any colors,
        /// but just a transformation matrix
        pub fn new() -> Self {
            Self {
                transform_matrix: Matrix::<4, 4>::identity(),
            }
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
}
