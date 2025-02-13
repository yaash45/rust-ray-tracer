use crate::{
    color::Color,
    matrix::{Matrix, Transformable},
    spatial::Tuple,
};

use super::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A pattern that consists of two colors, alternating in a checkerboard
/// pattern.  The pattern is centered at the origin, and repeats in the x, y, and
/// z directions.
pub struct Checker {
    a: Color,
    b: Color,
    transform_matrix: Matrix<4, 4>,
}

impl Checker {
    /// Create a new checker pattern with two colors and a transformation matrix.
    pub fn new(a: Color, b: Color, transform_matrix: Matrix<4, 4>) -> Self {
        Self {
            a,
            b,
            transform_matrix,
        }
    }
}

impl From<(Color, Color)> for Checker {
    fn from(value: (Color, Color)) -> Self {
        Self::new(value.0, value.1, Matrix::<4, 4>::identity())
    }
}

impl Transformable for Checker {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix
    }
}

impl Pattern for Checker {
    fn pattern_at(&self, point: &Tuple) -> Color {
        if (point.get_x().floor() + point.get_y().floor() + point.get_z().floor()) % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Checker, Pattern};
    use crate::{color::Color, spatial::Tuple};

    #[test]
    fn checker_pattern_tests() {
        let checker = Checker::from((Color::white(), Color::black()));

        // Checkers should repeat in x
        assert_eq!(checker.pattern_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(
            checker.pattern_at(&Tuple::point(0.99, 0, 0)),
            Color::white()
        );
        assert_eq!(
            checker.pattern_at(&Tuple::point(1.01, 0, 0)),
            Color::black()
        );

        // Checkers should repeat in y
        assert_eq!(checker.pattern_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(
            checker.pattern_at(&Tuple::point(0, 0.99, 0)),
            Color::white()
        );
        assert_eq!(
            checker.pattern_at(&Tuple::point(0, 1.01, 0)),
            Color::black()
        );

        // Checkers should repeat in z
        assert_eq!(checker.pattern_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(
            checker.pattern_at(&Tuple::point(0, 0, 0.99)),
            Color::white()
        );
        assert_eq!(
            checker.pattern_at(&Tuple::point(0, 0, 1.01)),
            Color::black()
        );
    }
}
