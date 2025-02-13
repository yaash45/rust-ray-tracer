mod solid;
mod striped;

use crate::{
    color::Color,
    matrix::{inverse_4x4, Transformable},
    shapes::Shape,
    spatial::Tuple,
};
use anyhow::Result;

pub use {solid::Solid, striped::Striped};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A enum representing all the different types of patterns
pub enum PatternType {
    Solid(Solid),
    Striped(Striped),
}

impl Pattern for PatternType {
    fn pattern_at(&self, point: &Tuple) -> Color {
        match self {
            PatternType::Solid(ref s) => s.pattern_at(point),
            PatternType::Striped(ref s) => s.pattern_at(point),
        }
    }

    fn pattern_at_object(&self, object: &Shape, world_point: &Tuple) -> Result<Color> {
        match self {
            PatternType::Solid(ref s) => s.pattern_at_object(object, world_point),
            PatternType::Striped(ref s) => s.pattern_at_object(object, world_point),
        }
    }
}

impl Transformable for PatternType {
    fn get_transform(&self) -> &crate::matrix::Matrix<4, 4> {
        match self {
            PatternType::Solid(ref s) => s.get_transform(),
            PatternType::Striped(ref s) => s.get_transform(),
        }
    }

    fn set_transform(&mut self, transform_matrix: crate::matrix::Matrix<4, 4>) {
        match self {
            PatternType::Solid(ref mut s) => s.set_transform(transform_matrix),
            PatternType::Striped(ref mut s) => s.set_transform(transform_matrix),
        }
    }
}

/// Trait for defining patterns with transformations.
/// Provides methods to get pattern color at a given point.
pub trait Pattern: Transformable {
    /// Return the color of the pattern at the given point.
    fn pattern_at(&self, point: &Tuple) -> Color;

    /// Given a shape and a point in the world, return the color of the pattern
    /// at the given point in the world. The pattern is transformed to the
    /// object's coordinate system before the color is determined.
    fn pattern_at_object(&self, object: &Shape, world_point: &Tuple) -> Result<Color> {
        let object_point = &inverse_4x4(object.get_transform())? * world_point;
        let pattern_point = &inverse_4x4(self.get_transform())? * &object_point;
        Ok(self.pattern_at(&pattern_point))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        matrix::{scaling, translation, Matrix, Transformable},
        spatial::Tuple,
        utils::test_utils::{PatternFactory, TestShapeFactory},
    };
    use anyhow::Result;

    use super::Pattern;

    #[test]
    fn test_default_pattern_transformation() {
        let pattern = PatternFactory::test_pattern();
        assert_eq!(pattern.get_transform(), &Matrix::<4, 4>::identity());
    }

    #[test]
    fn test_pattern_set_transformation() {
        let mut pattern = PatternFactory::test_pattern();
        pattern.set_transform(translation(1, 2, 3));
        assert_eq!(pattern.get_transform(), &translation(1, 2, 3));
    }

    #[test]
    fn test_pattern_with_object_transformation() -> Result<()> {
        let mut object = TestShapeFactory::test_shape();
        object.set_transform(scaling(2, 2, 2));

        let pattern = PatternFactory::test_pattern();
        let res = pattern.pattern_at_object(&object, &Tuple::point(2, 3, 4))?;

        assert_eq!(res, Color::new(1, 1.5, 2));

        Ok(())
    }

    #[test]
    fn test_a_pattern_with_pattern_transformation() -> Result<()> {
        let object = TestShapeFactory::test_shape();

        let mut pattern = PatternFactory::test_pattern();
        pattern.set_transform(scaling(2, 2, 2));

        assert_eq!(
            pattern.pattern_at_object(&object, &Tuple::point(2, 3, 4))?,
            Color::new(1, 1.5, 2)
        );

        Ok(())
    }

    #[test]
    fn test_with_both_object_and_pattern_transform() -> Result<()> {
        let mut object = TestShapeFactory::test_shape();
        object.set_transform(scaling(2, 2, 2));

        let mut pattern = PatternFactory::test_pattern();
        pattern.set_transform(translation(0.5, 1, 1.5));

        assert_eq!(
            pattern.pattern_at_object(&object, &Tuple::point(2.5, 3, 3.5))?,
            Color::new(0.75, 0.5, 0.25)
        );

        Ok(())
    }
}
