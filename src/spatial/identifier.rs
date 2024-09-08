use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Simple enum to differentiate between different types of spatial propertries
///
/// The two valid properties are `Points` and `Vectors`, but we also include an
/// `Invalid` option, to identify representations that cannot be categorized as
/// either points or vectors in 3D space.
pub enum Identifier {
    Vector = 0,
    Point = 1,
    Invalid = 2,
}

impl Identifier {
    /// Useful helper method to return an integer value representing
    /// the variant of the Identifier. This value corresponds with the
    /// `w` value associated with a spatial tuple. A `Point`'s `w`
    /// equals `1`, while a `Vector`'s equals `0`.
    ///
    /// We have made an arbitrary decision to represent an invalid
    /// identifier's value as the integer 2. It could have been
    /// any value outside the set `S = {0, 1}`, since those are the
    /// only two meaningful values in the context of the `w` value
    /// for spatial tuples.
    ///
    /// ```
    /// use raytracer::spatial::Identifier;
    ///
    /// let point_identifier = Identifier::Point;
    /// assert_eq!(point_identifier.value(), 1);
    ///
    /// let vector_identifier = Identifier::Vector;
    /// assert_eq!(vector_identifier.value(), 0);
    ///
    /// let invalid_identifier = Identifier::Invalid;
    /// assert_eq!(invalid_identifier.value(), 2);
    /// ```
    pub fn value(&self) -> isize {
        match *self {
            Self::Point => Self::Point as isize,
            Self::Vector => Self::Vector as isize,
            Self::Invalid => Self::Invalid as isize,
        }
    }
}

impl ops::Add<&Identifier> for &Identifier {
    type Output = Identifier;

    fn add(self, rhs: &Identifier) -> Self::Output {
        match self.value() + rhs.value() {
            0 => Identifier::Vector,
            1 => Identifier::Point,
            _ => Identifier::Invalid,
        }
    }
}

impl ops::Sub<&Identifier> for &Identifier {
    type Output = Identifier;

    fn sub(self, rhs: &Identifier) -> Self::Output {
        match self.value() - rhs.value() {
            0 => Identifier::Vector,
            1 => Identifier::Point,
            _ => Identifier::Invalid,
        }
    }
}

impl ops::Neg for &Identifier {
    type Output = Identifier;

    fn neg(self) -> Self::Output {
        match -(self.value()) {
            0 => Identifier::Vector,
            1 => Identifier::Point,
            _ => Identifier::Invalid,
        }
    }
}

impl ops::Mul<f64> for &Identifier {
    type Output = Identifier;

    fn mul(self, rhs: f64) -> Self::Output {
        match rhs * self.value() as f64 {
            0.0 => Identifier::Vector,
            1.0 => Identifier::Point,
            _ => Identifier::Invalid,
        }
    }
}

impl ops::Div<f64> for &Identifier {
    type Output = Identifier;

    fn div(self, rhs: f64) -> Self::Output {
        match (self.value() as f64) / rhs {
            0.0 => Identifier::Vector,
            1.0 => Identifier::Point,
            _ => Identifier::Invalid,
        }
    }
}

impl<T> From<T> for Identifier
where
    T: Into<f64>,
{
    fn from(value: T) -> Self {
        match value.into() {
            0.0 => Self::Vector,
            1.0 => Self::Point,
            _ => Self::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Identifier;

    #[test]
    fn test_value_getter() {
        // Check that the value method works as expected
        assert_eq!(Identifier::Point.value(), 1);
        assert_eq!(Identifier::Vector.value(), 0);
        assert_ne!(Identifier::Invalid.value(), 1);
        assert_ne!(Identifier::Invalid.value(), 0);
    }

    #[test]
    fn test_add() {
        let point = Identifier::Point;
        let vector = Identifier::Vector;

        assert_eq!(&point + &point, Identifier::Invalid);
        assert_eq!(&point + &vector, Identifier::Point);
        assert_eq!(&vector + &vector, Identifier::Vector);
    }

    #[test]
    fn test_sub() {
        let point = Identifier::Point;
        let vector = Identifier::Vector;

        assert_eq!(&point - &point, Identifier::Vector);
        assert_eq!(&point - &vector, Identifier::Point);
        assert_eq!(&vector - &vector, Identifier::Vector);
        assert_eq!(&vector - &point, Identifier::Invalid);
    }

    #[test]
    fn test_mul() {
        let vector = Identifier::Vector;
        let one: f64 = 1.0;
        let zero: f64 = 1.0;

        assert_eq!(&vector * one, Identifier::Vector);
        assert_eq!(&vector * zero, Identifier::Vector);
    }
}
