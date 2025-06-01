mod float_equals;

pub use float_equals::{float_equals, EPSILON};

#[cfg(test)]
pub mod test_utils {
    use crate::{
        patterns::{Pattern, TestPattern},
        shapes::{Shape, Sphere},
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
}
