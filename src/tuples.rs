use crate::spatial_identifier::SpatialIdentifier;
use std::ops;

#[derive(Clone, Debug, PartialEq)]
/// Data representing a spatial property like a Vector, or Point
pub struct SpatialTuple {
    x: f64,
    y: f64,
    z: f64,
    w: SpatialIdentifier,
}

impl SpatialTuple {
    /// Create a new Tuple
    pub fn new(
        x: impl Into<f64>,
        y: impl Into<f64>,
        z: impl Into<f64>,
        w: SpatialIdentifier,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w,
        }
    }

    /// Create a new Tuple of type Point, representing a point in 3D space
    pub fn new_point(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::new(x.into(), y.into(), z.into(), SpatialIdentifier::Point)
    }

    /// Create a new Tuple of type Vector, representing a vector in 3D space
    pub fn new_vector(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::new(x.into(), y.into(), z.into(), SpatialIdentifier::Vector)
    }

    /// Returns true if the tuple represents a Point in 3D space
    pub fn is_a_point(&self) -> bool {
        self.w == SpatialIdentifier::Point
    }

    /// Returns true if the tuple represents a Vector in 3D space
    pub fn is_a_vector(&self) -> bool {
        self.w == SpatialIdentifier::Vector
    }

    /// Returns a value representing the magnitude of the tuple using the
    /// formula: `magnitude = sqrt(x^2 + y^2 + z^2 + w^2)`
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(
            (self.x).powi(2)
                + (self.y).powi(2)
                + (self.z).powi(2)
                + (self.w.value() as f64).powi(2),
        )
    }

    /// Returns a new Tuple after converting the current tuple into a
    /// unit vector
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        self / magnitude
    }

    /// Returns a scalar value representing the dot product of two tuples
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
            + ((self.w.value() * other.w.value()) as f64)
    }

    /// Returns a vector representing cross product of the two inputs
    pub fn cross(&self, other: &Self) -> Self {
        let new_x = (self.y * other.z) - (self.z * other.y);
        let new_y = (self.z * other.x) - (self.x * other.z);
        let new_z = (self.x * other.y) - (self.y * other.x);

        Self::new_vector(new_x, new_y, new_z)
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }

    pub fn get_w(&self) -> f64 {
        self.w.value() as f64
    }
}

impl ops::Add<&SpatialTuple> for &SpatialTuple {
    type Output = SpatialTuple;

    fn add(self, rhs: &SpatialTuple) -> Self::Output {
        SpatialTuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: &self.w + &rhs.w,
        }
    }
}

impl ops::Sub<&SpatialTuple> for &SpatialTuple {
    type Output = SpatialTuple;

    fn sub(self, rhs: &SpatialTuple) -> Self::Output {
        SpatialTuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: &self.w - &rhs.w,
        }
    }
}

impl ops::Neg for &SpatialTuple {
    type Output = SpatialTuple;

    fn neg(self) -> Self::Output {
        SpatialTuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -&self.w,
        }
    }
}

impl ops::Mul<f64> for &SpatialTuple {
    type Output = SpatialTuple;

    fn mul(self, rhs: f64) -> Self::Output {
        SpatialTuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: &self.w * rhs,
        }
    }
}

impl ops::Div<f64> for &SpatialTuple {
    type Output = SpatialTuple;

    fn div(self, rhs: f64) -> Self::Output {
        SpatialTuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: &self.w / rhs,
        }
    }
}

impl<T, U, G, N> From<(T, U, G, N)> for SpatialTuple
where
    T: Into<f64>,
    U: Into<f64>,
    G: Into<f64>,
    N: Into<f64>,
{
    fn from(value: (T, U, G, N)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
            w: SpatialIdentifier::from(value.3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpatialTuple;
    use crate::spatial_identifier::SpatialIdentifier;

    #[test]
    fn tuple_new() {
        let tuple = SpatialTuple::new(4.3, -4.2, 3.1, SpatialIdentifier::Point);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w.value(), 1);

        let tuple = SpatialTuple::new(4.3, -4.2, 3.1, SpatialIdentifier::Vector);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w.value(), 0);
    }

    #[test]
    /// Tests if the function correctly flags
    /// if a tuple is a point and not a vector
    fn tuple_is_a_point() {
        let tuple = SpatialTuple::from((4.3, -4.2, 3.1, 1));
        assert!(tuple.is_a_point());
        assert!(!tuple.is_a_vector());
    }

    #[test]
    /// Tests if the function correctly flags
    /// if a tuple is a vector and not a point
    fn tuple_is_a_vector() {
        let tuple = SpatialTuple::from((4.3, -4.2, 3.1, 0));
        assert!(!tuple.is_a_point());
        assert!(tuple.is_a_vector());
    }

    #[test]
    fn create_point_from_tuple() {
        let x = 4_f64;
        let y = -4_f64;
        let z = 3_f64;

        let expected = SpatialTuple::new(x, y, z, SpatialIdentifier::Point);
        let actual = SpatialTuple::new_point(x, y, z);
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_vector_from_tuple() {
        let x = 4_f64;
        let y = -4_f64;
        let z = 3_f64;

        let expected = SpatialTuple::new(x, y, z, SpatialIdentifier::Vector);
        let actual = SpatialTuple::new_vector(x, y, z);
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_tuples() {
        let point_a = SpatialTuple::new_point(3.0, -2.0, 5.0);
        let point_b = SpatialTuple::new_point(3.0, -2.0, 5.0);
        let vector_a = SpatialTuple::new_vector(-2.0, 3.0, 1.0);
        let vector_b = SpatialTuple::new_vector(-2.0, -3.0, 1.0);

        // Adding a point and a vector must yield a point
        let expected = SpatialTuple::new_point(1.0, 1.0, 6.0);
        let actual = &point_a + &vector_a;
        assert!(actual.is_a_point());
        assert_eq!(expected, actual);

        // Adding two vectors must yield a vector
        let expected = SpatialTuple::new_vector(-4.0, 0.0, 2.0);
        let actual = &vector_a + &vector_b;
        assert!(actual.is_a_vector());
        assert_eq!(expected, actual);

        // Adding two points must yield an "invalid" spatial tuple
        let expected = SpatialTuple::new(6.0, -4.0, 10.0, SpatialIdentifier::Invalid);
        let actual = &point_a + &point_b;
        assert!(!actual.is_a_point());
        assert!(!actual.is_a_vector());
        assert_eq!(expected, actual);
    }

    #[test]
    fn subtract_tuples() {
        let point_a = SpatialTuple::new_point(3.0, -2.0, 5.0);
        let point_b = SpatialTuple::new_point(1.0, 4.0, 3.0);
        let vector_a = SpatialTuple::new_vector(-2.0, 3.0, 1.0);
        let vector_b = SpatialTuple::new_vector(5.0, -3.0, -5.0);

        // Adding a point and a vector must yield a point
        let expected = SpatialTuple::new_point(5.0, -5.0, 4.0);
        let actual = &point_a - &vector_a;
        assert!(actual.is_a_point());
        assert_eq!(expected, actual);

        // Adding two vectors must yield a vector
        let expected = SpatialTuple::new_vector(-7.0, 6.0, 6.0);
        let actual = &vector_a - &vector_b;
        assert!(actual.is_a_vector());
        assert_eq!(expected, actual);

        // Adding two points must yield an "invalid" spatial tuple
        let expected = SpatialTuple::new(-3.0, -1.0, -2.0, SpatialIdentifier::Invalid);
        let actual = &vector_a - &point_b;
        assert!(!actual.is_a_point());
        assert!(!actual.is_a_vector());
        assert_eq!(expected, actual);
    }

    #[test]
    fn negate_tuples() {
        let point_a = SpatialTuple::new_point(3.0, -2.0, 5.0);
        let vector_a = SpatialTuple::new_vector(-2.0, 3.0, 1.0);

        // Negating a vector should yield a vector with the coordinates negated
        let expected = SpatialTuple::new_vector(2.0, -3.0, -1.0);
        let actual = -&vector_a;
        assert!(actual.is_a_vector());
        assert_eq!(actual, expected);

        // Negating a point should yield an invalid spatial tuple
        let expected = SpatialTuple::new(-3.0, 2.0, -5.0, SpatialIdentifier::Invalid);
        let actual = -&point_a;
        assert!(!actual.is_a_point());
        assert!(!actual.is_a_vector());
        assert_eq!(actual, expected);
    }

    #[test]
    fn scale_vectors() {
        let vector_a = SpatialTuple::new_vector(-2.0, 3.0, 1.0);
        let scalar = 2.0;

        // Check multiplication scaling
        let expected = SpatialTuple::new_vector(-4.0, 6.0, 2.0);
        let actual = &vector_a * scalar;
        assert!(actual.is_a_vector());
        assert_eq!(actual, expected);

        // Check division scaling
        let expected = SpatialTuple::new_vector(-1.0, 1.5, 0.5);
        let actual = &vector_a / scalar;
        assert!(actual.is_a_vector());
        assert_eq!(actual, expected);
    }

    #[test]
    fn magnitude() {
        let vector_a = SpatialTuple::new_vector(1, 0, 0);
        assert_eq!(vector_a.magnitude(), 1.0);

        let vector_b = SpatialTuple::new_vector(0, 1, 0);
        assert_eq!(vector_b.magnitude(), 1.0);

        let vector_c = SpatialTuple::new_vector(0, 0, 1);
        assert_eq!(vector_c.magnitude(), 1.0);

        let vector_d = SpatialTuple::new_vector(1, 2, 3);
        assert_eq!(vector_d.magnitude(), f64::sqrt(14.0));

        let vector_e = SpatialTuple::new_vector(-1, -2, -3);
        assert_eq!(vector_e.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalize() {
        let vector_a = SpatialTuple::new_vector(4, 0, 0);
        assert_eq!(vector_a.normalize(), SpatialTuple::new_vector(1, 0, 0));

        let vector_b = SpatialTuple::new_vector(1, 2, 3);
        let normalized_b = vector_b.normalize();
        assert_eq!(
            normalized_b,
            SpatialTuple::new_vector(
                1.0 / f64::sqrt(14.0),
                2.0 / f64::sqrt(14.0),
                3.0 / f64::sqrt(14.0)
            )
        );
        assert_eq!(normalized_b.magnitude(), 1.0);
    }

    #[test]
    fn dot() {
        let a = SpatialTuple::new_vector(1, 2, 3);
        let b = SpatialTuple::new_vector(2, 3, 4);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross() {
        let a = SpatialTuple::new_vector(1, 2, 3);
        let b = SpatialTuple::new_vector(2, 3, 4);

        assert_eq!(a.cross(&b), SpatialTuple::new_vector(-1, 2, -1));
        assert_eq!(b.cross(&a), SpatialTuple::new_vector(1, -2, 1));
    }
}
