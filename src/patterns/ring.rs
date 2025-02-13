use crate::{
    color::Color,
    matrix::{Matrix, Transformable},
    spatial::Tuple,
};

use super::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A pattern that alternates between two colors in concentric rings.
/// The pattern is centered at the origin and repeats in the x and z directions.
pub struct Ring {
    a: Color,
    b: Color,
    transform_matrix: Matrix<4, 4>,
}

impl Ring {
    /// Create a new ring pattern with two colors and a transformation matrix.
    pub fn new(a: Color, b: Color, transform_matrix: Matrix<4, 4>) -> Self {
        Self {
            a,
            b,
            transform_matrix,
        }
    }
}

impl From<(Color, Color)> for Ring {
    fn from(value: (Color, Color)) -> Self {
        Self::new(value.0, value.1, Matrix::<4, 4>::identity())
    }
}

impl Transformable for Ring {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix
    }
}

impl Pattern for Ring {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let val = point.get_x().powi(2) + point.get_z().powi(2);

        if val.sqrt().floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Pattern, Ring};
    use crate::{color::Color, spatial::Tuple};

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let ring = Ring::from((Color::white(), Color::black()));

        // Test pattern_at at different points
        assert_eq!(ring.pattern_at(&Tuple::point(0, 0, 0)), Color::white());
        assert_eq!(ring.pattern_at(&Tuple::point(1, 0, 0)), Color::black());
        assert_eq!(ring.pattern_at(&Tuple::point(0, 0, 1)), Color::black());
        // 0.708 is just slightly more than sqrt(2)
        assert_eq!(
            ring.pattern_at(&Tuple::point(0.708, 0, 0.708)),
            Color::black()
        );
    }
}
