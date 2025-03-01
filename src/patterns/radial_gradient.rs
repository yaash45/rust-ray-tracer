use crate::{
    color::Color,
    matrix::{Matrix, Transformable},
    spatial::Tuple,
};

use super::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A pattern that smoothly transitions between two colors and repeats in a ring pattern.
/// This pattern combines the gradient and ring patterns, creating a ring pattern
/// with a gradient of colors.
pub struct GradientRing {
    a: Color,
    b: Color,
    transform_matrix: Matrix<4, 4>,
}

impl GradientRing {
    /// Create a new gradient ring pattern with two colors and a transformation matrix.
    ///
    /// The gradient ring pattern combines the gradient and ring patterns, creating a ring pattern
    /// with a gradient of colors. The pattern is centered at the origin, and repeats in the x and z
    /// directions.
    ///
    /// The `a` color is the color at x=0 and z=0, and the `b` color is the color at x=1 and z=1.
    /// The color at any other x and z values is a linear interpolation between `a` and `b`.
    pub fn new(a: Color, b: Color, transform_matrix: Matrix<4, 4>) -> Self {
        Self {
            a,
            b,
            transform_matrix,
        }
    }
}

impl From<(Color, Color)> for GradientRing {
    fn from(value: (Color, Color)) -> Self {
        Self::new(value.0, value.1, Matrix::<4, 4>::identity())
    }
}

impl Transformable for GradientRing {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix
    }
}

impl Pattern for GradientRing {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let val = point.get_x().powi(2) + point.get_z().powi(2);
        let fraction = point.get_x() - point.get_x().floor();

        if val.sqrt().floor() % 2.0 == 0.0 {
            let distance = self.b - self.a;
            self.a + (distance * fraction)
        } else {
            let distance = self.a - self.b;
            self.b + (distance * fraction)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GradientRing, Pattern};
    use crate::{color::Color, spatial::Tuple};

    #[test]
    fn gradient_ring_pattern_at() {
        let ring = GradientRing::from((Color::white(), Color::black()));

        // Test pattern_at at different points
        assert_eq!(ring.pattern_at(&Tuple::point(0, 0, 0)), Color::white());

        // Test gradient rings in the x direction
        assert_eq!(
            ring.pattern_at(&Tuple::point(0.25, 0, 0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            ring.pattern_at(&Tuple::point(0.75, 0, 0)),
            Color::new(0.25, 0.25, 0.25)
        );
        assert_eq!(ring.pattern_at(&Tuple::point(1, 0, 0)), Color::black());
        assert_eq!(
            ring.pattern_at(&Tuple::point(1.25, 0, 0)),
            Color::new(0.25, 0.25, 0.25)
        );

        // Test gradient rings in the z direction
        assert_eq!(ring.pattern_at(&Tuple::point(0, 0, 0.25)), Color::white());
        assert_eq!(ring.pattern_at(&Tuple::point(0, 0, 1)), Color::black());
    }
}
