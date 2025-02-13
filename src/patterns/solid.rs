use super::Pattern;
use crate::{
    color::Color,
    matrix::{Matrix, Transformable},
    shapes::Shape,
    spatial::Tuple,
};
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A solid color pattern, useful for creating basic shapes in a scene.
/// Solid patterns have a single color, and are constant in space.
pub struct Solid {
    color: Color,
    transform_matrix: Matrix<4, 4>,
}

impl Solid {
    /// Create a new solid pattern with a single color.
    pub fn new(color: Color) -> Self {
        Self {
            color,
            transform_matrix: Matrix::<4, 4>::identity(),
        }
    }
}

impl From<Color> for Solid {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}

impl Transformable for Solid {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix
    }
}

impl Pattern for Solid {
    fn pattern_at(&self, _point: &Tuple) -> Color {
        self.color
    }

    fn pattern_at_object(&self, _object: &Shape, world_point: &Tuple) -> Result<Color> {
        Ok(self.pattern_at(world_point))
    }
}

#[cfg(test)]
mod tests {
    use super::{Pattern, Solid};
    use crate::{
        color::Color,
        matrix::{scaling, Transformable},
        spatial::Tuple,
        utils::test_utils::TestShapeFactory,
    };
    use anyhow::Result;

    #[test]
    fn test_pattern_at_returns_same_color_throughout() {
        let solid = Solid::new(Color::new(1, 1, 1));

        assert_eq!(
            solid.pattern_at(&Tuple::point(0, 0, 0)),
            Color::new(1, 1, 1)
        );

        assert_eq!(
            solid.pattern_at(&Tuple::point(0.5, 0.5, 0.5)),
            Color::new(1, 1, 1)
        );

        assert_eq!(
            solid.pattern_at(&Tuple::point(1, 1, 1)),
            Color::new(1, 1, 1)
        )
    }

    #[test]
    fn test_pattern_at_object_is_unaffected_by_transformations() -> Result<()> {
        let mut object = TestShapeFactory::test_shape();
        object.set_transform(scaling(0.4, 0.5, 0.2));

        let solid = Solid::new(Color::new(1, 1, 1));

        assert_eq!(
            solid.pattern_at_object(&object, &Tuple::point(0, 0, 0))?,
            Color::new(1, 1, 1)
        );

        assert_eq!(
            solid.pattern_at_object(&object, &Tuple::point(0.5, 0.5, 0.5))?,
            Color::new(1, 1, 1)
        );

        assert_eq!(
            solid.pattern_at_object(&object, &Tuple::point(1, 1, 1))?,
            Color::new(1, 1, 1)
        );

        Ok(())
    }
}
