use crate::{
    color::Color,
    matrix::{Matrix, Transformable},
    spatial::Tuple,
};

use super::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A pattern that smoothly transitions between two colors.
/// The color at any point is the linear interpolation of the two colors.
pub struct Gradient {
    a: Color,
    b: Color,
    transform_matrix: Matrix<4, 4>,
}

impl Gradient {
    /// Create a new gradient pattern with two colors and a transformation matrix.
    ///
    /// The `a` color is the color at x=0 and the `b` color is the color at x=1.
    /// The color at any other x value is a linear interpolation between `a` and `b`.
    ///
    pub fn new(a: Color, b: Color, transform_matrix: Matrix<4, 4>) -> Self {
        Self {
            a,
            b,
            transform_matrix,
        }
    }
}

impl From<(Color, Color)> for Gradient {
    fn from(value: (Color, Color)) -> Self {
        Self::new(value.0, value.1, Matrix::<4, 4>::identity())
    }
}

impl Transformable for Gradient {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix
    }
}

impl Pattern for Gradient {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.get_x() - point.get_x().floor();

        self.a + (distance * fraction)
    }
}

#[cfg(test)]
mod tests {
    use super::{Gradient, Pattern};
    use crate::{color::Color, spatial::Tuple};

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let gradient = Gradient::from((Color::white(), Color::black()));

        // Test pattern_at at different points
        assert_eq!(gradient.pattern_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(
            gradient.pattern_at(&Tuple::point(0.25, 0, 0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            gradient.pattern_at(&Tuple::point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            gradient.pattern_at(&Tuple::point(0.75, 0, 0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
