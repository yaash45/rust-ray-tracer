use super::identifier::Identifier;
use crate::utils::float_equals;
use std::ops;

#[derive(Clone, Copy, Debug)]
/// Representation of a spatial property like a Vector, or Point
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: Identifier,
}

impl Tuple {
    /// Create a new [Tuple]
    ///
    /// ```
    /// use raytracer::spatial::{Tuple, Identifier};
    ///
    /// // Creating a point (0,0,0)
    /// let point = Tuple::new(0,0,0, Identifier::Point);
    ///
    /// // Creating a vector (0,1,0)
    /// let vector = Tuple::new(0,1,0, Identifier::Vector);
    /// ```
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>, w: Identifier) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w,
        }
    }

    /// Create a new [Tuple] of type Point, representing a point in 3D space.
    ///
    /// ```
    /// use raytracer::spatial::Tuple;
    ///
    /// // Create a new point (1,1,1)
    /// let point = Tuple::point(1,1,1);
    /// ```
    pub fn point(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::new(x.into(), y.into(), z.into(), Identifier::Point)
    }

    /// Create a new [Tuple] of type Vector, representing a vector in 3D space
    ///
    /// ```
    /// use raytracer::spatial::Tuple;
    ///
    /// // Create a new vector (1,1,1)
    /// let vector = Tuple::vector(1,1,1);
    /// ```
    pub fn vector(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::new(x.into(), y.into(), z.into(), Identifier::Vector)
    }

    /// Returns true if the [Tuple] represents a Point in 3D space
    ///
    /// ```
    /// use raytracer::spatial::Tuple;
    ///
    /// // Create a new point
    /// let p = Tuple::point(0,0,0);
    /// assert!(p.is_a_point());
    /// assert!(!p.is_a_vector());
    /// ```
    pub fn is_a_point(&self) -> bool {
        self.w == Identifier::Point
    }

    /// Returns true if the [Tuple] represents a Vector in 3D space
    ///
    /// ```
    /// use raytracer::spatial::Tuple;
    ///
    /// // Create a new vector
    /// let v = Tuple::vector(0,1.4,6.5);
    /// assert!(!v.is_a_point());
    /// assert!(v.is_a_vector());
    /// ```
    pub fn is_a_vector(&self) -> bool {
        self.w == Identifier::Vector
    }

    /// Returns a value representing the magnitude of the [Tuple] using the
    /// formula: magnitude = squareRoot(x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> + w<sup>2</sup>)
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(
            (self.x).powi(2)
                + (self.y).powi(2)
                + (self.z).powi(2)
                + (self.w.value() as f64).powi(2),
        )
    }

    /// Returns a new [Tuple] after converting the current [Tuple] into a
    /// unit vector using the formula:
    /// normalize(v) = v ÷ magnitude(v)
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        self / magnitude
    }

    /// Returns a scalar value representing the dot product of two [Tuple]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
            + ((self.w.value() * other.w.value()) as f64)
    }

    /// Returns a vector representing cross product of this [Tuple] with an input [Tuple]
    ///
    /// _Remember: the order of [Tuple] inputs matters in the case of a cross product,
    /// aka `A x B` is not necessarily  equal to `B x A`_
    pub fn cross(&self, other: &Self) -> Self {
        let new_x = (self.y * other.z) - (self.z * other.y);
        let new_y = (self.z * other.x) - (self.x * other.z);
        let new_z = (self.x * other.y) - (self.y * other.x);

        Self::vector(new_x, new_y, new_z)
    }

    /// Returns the x coordinate of the [Tuple]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Returns the y coordinate of the [Tuple]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    /// Returns the z coordinate of the [Tuple]
    pub fn get_z(&self) -> f64 {
        self.z
    }

    /// Returns the w coordinate of the [Tuple]
    pub fn get_w(&self) -> f64 {
        self.w.value() as f64
    }

    /// Returns a vector with the x,y,z values
    /// of the current [Tuple]
    pub fn convert_to_vector(&self) -> Tuple {
        Tuple::vector(self.x, self.y, self.z)
    }
}

impl ops::Add<&Tuple> for &Tuple {
    type Output = Tuple;

    fn add(self, rhs: &Tuple) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: &self.w + &rhs.w,
        }
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: &self.w + &rhs.w,
        }
    }
}

impl ops::Sub<&Tuple> for &Tuple {
    type Output = Tuple;

    fn sub(self, rhs: &Tuple) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: &self.w - &rhs.w,
        }
    }
}

impl ops::Neg for &Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -&self.w,
        }
    }
}

impl ops::Mul<f64> for &Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: &self.w * rhs,
        }
    }
}

impl ops::Div<f64> for &Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: &self.w / rhs,
        }
    }
}

impl<T, U, G, N> From<(T, U, G, N)> for Tuple
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
            w: Identifier::from(value.3),
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_equals(&self.x, &other.x)
            && float_equals(&self.y, &other.y)
            && float_equals(&self.z, &other.z)
            && self.w == other.w
    }
}

#[cfg(test)]
mod tests {
    use super::Identifier;
    use super::Tuple;

    #[test]
    fn tuple_new() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, Identifier::Point);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w.value(), 1);

        let tuple = Tuple::new(4.3, -4.2, 3.1, Identifier::Vector);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w.value(), 0);
    }

    #[test]
    /// Tests if the function correctly flags
    /// if a tuple is a point and not a vector
    fn tuple_is_a_point() {
        let tuple = Tuple::from((4.3, -4.2, 3.1, 1));
        assert!(tuple.is_a_point());
        assert!(!tuple.is_a_vector());
    }

    #[test]
    /// Tests if the function correctly flags
    /// if a tuple is a vector and not a point
    fn tuple_is_a_vector() {
        let tuple = Tuple::from((4.3, -4.2, 3.1, 0));
        assert!(!tuple.is_a_point());
        assert!(tuple.is_a_vector());
    }

    #[test]
    fn create_point_from_tuple() {
        let x = 4_f64;
        let y = -4_f64;
        let z = 3_f64;

        let expected = Tuple::new(x, y, z, Identifier::Point);
        let actual = Tuple::point(x, y, z);
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_vector_from_tuple() {
        let x = 4_f64;
        let y = -4_f64;
        let z = 3_f64;

        let expected = Tuple::new(x, y, z, Identifier::Vector);
        let actual = Tuple::vector(x, y, z);
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_tuples() {
        let point_a = Tuple::point(3.0, -2.0, 5.0);
        let point_b = Tuple::point(3.0, -2.0, 5.0);
        let vector_a = Tuple::vector(-2.0, 3.0, 1.0);
        let vector_b = Tuple::vector(-2.0, -3.0, 1.0);

        // Adding a point and a vector must yield a point
        let expected = Tuple::point(1.0, 1.0, 6.0);
        let actual = point_a + vector_a;
        assert!(actual.is_a_point());
        assert_eq!(expected, actual);

        // Adding two vectors must yield a vector
        let expected = Tuple::vector(-4.0, 0.0, 2.0);
        let actual = vector_a + vector_b;
        assert!(actual.is_a_vector());
        assert_eq!(expected, actual);

        // Adding two points must yield an "invalid" spatial tuple
        let expected = Tuple::new(6.0, -4.0, 10.0, Identifier::Invalid);
        let actual = point_a + point_b;
        assert!(!actual.is_a_point());
        assert!(!actual.is_a_vector());
        assert_eq!(expected, actual);
    }

    #[test]
    fn subtract_tuples() {
        let point_a = Tuple::point(3.0, -2.0, 5.0);
        let point_b = Tuple::point(1.0, 4.0, 3.0);
        let vector_a = Tuple::vector(-2.0, 3.0, 1.0);
        let vector_b = Tuple::vector(5.0, -3.0, -5.0);

        // Adding a point and a vector must yield a point
        let expected = Tuple::point(5.0, -5.0, 4.0);
        let actual = &point_a - &vector_a;
        assert!(actual.is_a_point());
        assert_eq!(expected, actual);

        // Adding two vectors must yield a vector
        let expected = Tuple::vector(-7.0, 6.0, 6.0);
        let actual = &vector_a - &vector_b;
        assert!(actual.is_a_vector());
        assert_eq!(expected, actual);

        // Adding two points must yield an "invalid" spatial tuple
        let expected = Tuple::new(-3.0, -1.0, -2.0, Identifier::Invalid);
        let actual = &vector_a - &point_b;
        assert!(!actual.is_a_point());
        assert!(!actual.is_a_vector());
        assert_eq!(expected, actual);
    }

    #[test]
    fn negate_tuples() {
        let point_a = Tuple::point(3.0, -2.0, 5.0);
        let vector_a = Tuple::vector(-2.0, 3.0, 1.0);

        // Negating a vector should yield a vector with the coordinates negated
        let expected = Tuple::vector(2.0, -3.0, -1.0);
        let actual = -&vector_a;
        assert!(actual.is_a_vector());
        assert_eq!(actual, expected);

        // Negating a point should yield an invalid spatial tuple
        let expected = Tuple::new(-3.0, 2.0, -5.0, Identifier::Invalid);
        let actual = -&point_a;
        assert!(!actual.is_a_point());
        assert!(!actual.is_a_vector());
        assert_eq!(actual, expected);
    }

    #[test]
    fn scale_vectors() {
        let vector_a = Tuple::vector(-2.0, 3.0, 1.0);
        let scalar = 2.0;

        // Check multiplication scaling
        let expected = Tuple::vector(-4.0, 6.0, 2.0);
        let actual = &vector_a * scalar;
        assert!(actual.is_a_vector());
        assert_eq!(actual, expected);

        // Check division scaling
        let expected = Tuple::vector(-1.0, 1.5, 0.5);
        let actual = &vector_a / scalar;
        assert!(actual.is_a_vector());
        assert_eq!(actual, expected);
    }

    #[test]
    fn magnitude() {
        let vector_a = Tuple::vector(1, 0, 0);
        assert_eq!(vector_a.magnitude(), 1.0);

        let vector_b = Tuple::vector(0, 1, 0);
        assert_eq!(vector_b.magnitude(), 1.0);

        let vector_c = Tuple::vector(0, 0, 1);
        assert_eq!(vector_c.magnitude(), 1.0);

        let vector_d = Tuple::vector(1, 2, 3);
        assert_eq!(vector_d.magnitude(), f64::sqrt(14.0));

        let vector_e = Tuple::vector(-1, -2, -3);
        assert_eq!(vector_e.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalize() {
        let vector_a = Tuple::vector(4, 0, 0);
        assert_eq!(vector_a.normalize(), Tuple::vector(1, 0, 0));

        let vector_b = Tuple::vector(1, 2, 3);
        let normalized_b = vector_b.normalize();
        assert_eq!(
            normalized_b,
            Tuple::vector(
                1.0 / f64::sqrt(14.0),
                2.0 / f64::sqrt(14.0),
                3.0 / f64::sqrt(14.0)
            )
        );
        assert_eq!(normalized_b.magnitude(), 1.0);
    }

    #[test]
    fn dot() {
        let a = Tuple::vector(1, 2, 3);
        let b = Tuple::vector(2, 3, 4);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross() {
        let a = Tuple::vector(1, 2, 3);
        let b = Tuple::vector(2, 3, 4);

        assert_eq!(a.cross(&b), Tuple::vector(-1, 2, -1));
        assert_eq!(b.cross(&a), Tuple::vector(1, -2, 1));
    }

    #[test]
    fn convert_to_vector_works() {
        let p = Tuple::point(2, 3, 4);

        // case 1: point to vector
        assert_eq!(p.convert_to_vector(), Tuple::vector(2, 3, 4));

        let v = Tuple::vector(2, 4, 5);
        // case 2: vector to vector
        assert_eq!(v.convert_to_vector(), Tuple::vector(2, 4, 5));
    }
}
