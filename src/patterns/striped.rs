use super::Pattern;
use crate::{color::Color, matrix::Matrix, matrix::Transformable, spatial::Tuple};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A pattern that has two colors and repeats in a striped manner.
/// Striped patterns are useful for creating textures that have
/// a repeating pattern of two colors. For example, a zebra
/// pattern would be a striped pattern with black and white.
/// Striped patterns repeat in the x axis, meaning that the
/// color of the pattern changes as the x component of a point
/// changes.
pub struct Striped {
    a: Color,
    b: Color,
    transform_matrix: Matrix<4, 4>,
}

impl Striped {
    /// Create a new striped pattern with two colors. The color of the
    /// pattern changes as the x component of a point changes.
    pub fn new(a: Color, b: Color, transform_matrix: Matrix<4, 4>) -> Self {
        Self {
            a,
            b,
            transform_matrix,
        }
    }
}

impl Pattern for Striped {
    fn pattern_at(&self, point: &Tuple) -> Color {
        if point.get_x().floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Transformable for Striped {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix
    }
}

impl From<(Color, Color)> for Striped {
    /// Create a new striped pattern with two colors with the transform
    /// set to the identity matrix.
    ///
    /// The pattern alternates between `a` and `b` as the x component of a point
    /// changes.
    fn from(value: (Color, Color)) -> Self {
        Striped::new(value.0, value.1, Matrix::<4, 4>::identity())
    }
}

#[cfg(test)]
mod tests {
    use super::{Pattern, Striped};
    use crate::{
        color::Color,
        matrix::{scaling, translation, Matrix, Transformable},
        spatial::Tuple,
        utils::test_utils::TestShapeFactory,
    };
    use anyhow::Result;

    #[test]
    fn pattern_at_returns_correct_color_value() {
        let pattern = Striped::new(Color::white(), Color::black(), Matrix::<4, 4>::identity());

        // Stripe pattern is constant in y
        assert_eq!(pattern.pattern_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(pattern.pattern_at(&Tuple::point(0, 1, 0)), Color::white());
        assert_eq!(pattern.pattern_at(&Tuple::point(0, 2, 0)), Color::white());

        // Stripe pattern is constant in z
        assert_eq!(pattern.pattern_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(pattern.pattern_at(&Tuple::point(0, 0, 1)), Color::white());
        assert_eq!(pattern.pattern_at(&Tuple::point(0, 0, 2)), Color::white());

        // Stripe pattern can change in the x direction
        assert_eq!(pattern.pattern_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(pattern.pattern_at(&Tuple::point(0.9, 0, 0)), Color::white());
        assert_eq!(pattern.pattern_at(&Tuple::point(1, 0, 0)), Color::black());
        assert_eq!(
            pattern.pattern_at(&Tuple::point(-0.1, 0, 0)),
            Color::black()
        );
        assert_eq!(pattern.pattern_at(&Tuple::point(-1, 0, 0)), Color::black());
        assert_eq!(
            pattern.pattern_at(&Tuple::point(-1.1, 0, 0)),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_object_transformation() -> Result<()> {
        let mut object = TestShapeFactory::test_shape();
        object.set_transform(scaling(2, 2, 2));

        let pattern = Striped::from((Color::white(), Color::black()));

        assert_eq!(
            pattern.pattern_at_object(&object, &Tuple::point(1.5, 0, 0))?,
            Color::white()
        );

        Ok(())
    }

    #[test]
    fn stripes_with_pattern_transformation() -> Result<()> {
        let object = TestShapeFactory::test_shape();

        let mut pattern = Striped::from((Color::white(), Color::black()));
        pattern.set_transform(scaling(2, 2, 2));

        assert_eq!(
            pattern.pattern_at_object(&object, &Tuple::point(1.5, 0, 0))?,
            Color::white()
        );

        Ok(())
    }

    #[test]
    fn stripes_with_object_and_pattern_transformation() -> Result<()> {
        let mut object = TestShapeFactory::test_shape();
        object.set_transform(scaling(2, 2, 2));

        let mut pattern = Striped::from((Color::white(), Color::black()));
        pattern.set_transform(translation(0.5, 0, 0));

        assert_eq!(
            pattern.pattern_at_object(&object, &Tuple::point(2.5, 0, 0))?,
            Color::white()
        );

        Ok(())
    }
}
