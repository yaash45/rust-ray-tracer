use crate::{
    color::Color,
    matrix::{Matrix, Transformable},
    spatial::Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A pattern that has two colors and repeats in a striped manner.
/// Striped patterns are useful for creating textures that have
/// a repeating pattern of two colors. For example, a zebra
/// pattern would be a striped pattern with black and white.
/// Striped patterns repeat in the x axis, meaning that the
/// color of the pattern changes as the x component of a point
/// changes.
pub struct StripedPattern {
    a: Color,
    b: Color,
    transform_matrix: Matrix<4, 4>,
}

impl StripedPattern {
    /// Create a new striped pattern with two colors. The color of the
    /// pattern changes as the x component of a point changes.
    pub fn new(a: Color, b: Color, transform_matrix: Matrix<4, 4>) -> Self {
        Self {
            a,
            b,
            transform_matrix,
        }
    }

    /// Return the color of the pattern at the given point.
    /// The pattern alternates between `a` and `b` as the x
    /// component of the point changes.
    pub fn stripe_at(&self, point: &Tuple) -> Color {
        if point.get_x().floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Transformable for StripedPattern {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::StripedPattern;
    use crate::{color::Color, matrix::Matrix, spatial::Tuple};

    #[test]
    fn stripe_at_returns_correct_color_value() {
        let pattern =
            StripedPattern::new(Color::white(), Color::black(), Matrix::<4, 4>::identity());

        // Stripe pattern is constant in y
        assert_eq!(pattern.stripe_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(pattern.stripe_at(&Tuple::point(0, 1, 0)), Color::white());
        assert_eq!(pattern.stripe_at(&Tuple::point(0, 2, 0)), Color::white());

        // Stripe pattern is constant in z
        assert_eq!(pattern.stripe_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(pattern.stripe_at(&Tuple::point(0, 0, 1)), Color::white());
        assert_eq!(pattern.stripe_at(&Tuple::point(0, 0, 2)), Color::white());

        // Stripe pattern can change in the x direction
        assert_eq!(pattern.stripe_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(pattern.stripe_at(&Tuple::point(0.9, 0, 0)), Color::white());
        assert_eq!(pattern.stripe_at(&Tuple::point(1, 0, 0)), Color::black());
        assert_eq!(pattern.stripe_at(&Tuple::point(-0.1, 0, 0)), Color::black());
        assert_eq!(pattern.stripe_at(&Tuple::point(-1, 0, 0)), Color::black());
        assert_eq!(pattern.stripe_at(&Tuple::point(-1.1, 0, 0)), Color::white());
    }
}
