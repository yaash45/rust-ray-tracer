use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub enum SpatialIdentifier {
    Vector = 0,
    Point = 1,
    Invalid = 2,
}

impl SpatialIdentifier {
    pub fn value(&self) -> isize {
        match *self {
            Self::Point => Self::Point as isize,
            Self::Vector => Self::Vector as isize,
            Self::Invalid => Self::Invalid as isize,
        }
    }
}

impl ops::Add<&SpatialIdentifier> for &SpatialIdentifier {
    type Output = SpatialIdentifier;

    fn add(self, rhs: &SpatialIdentifier) -> Self::Output {
        match self.value() + rhs.value() {
            0 => SpatialIdentifier::Vector,
            1 => SpatialIdentifier::Point,
            _ => SpatialIdentifier::Invalid,
        }
    }
}

impl ops::Sub<&SpatialIdentifier> for &SpatialIdentifier {
    type Output = SpatialIdentifier;

    fn sub(self, rhs: &SpatialIdentifier) -> Self::Output {
        match self.value() - rhs.value() {
            0 => SpatialIdentifier::Vector,
            1 => SpatialIdentifier::Point,
            _ => SpatialIdentifier::Invalid,
        }
    }
}

impl ops::Neg for &SpatialIdentifier {
    type Output = SpatialIdentifier;

    fn neg(self) -> Self::Output {
        match -(self.value()) {
            0 => SpatialIdentifier::Vector,
            1 => SpatialIdentifier::Point,
            _ => SpatialIdentifier::Invalid,
        }
    }
}

impl ops::Mul<f64> for &SpatialIdentifier {
    type Output = SpatialIdentifier;

    fn mul(self, rhs: f64) -> Self::Output {
        match rhs * self.value() as f64 {
            0.0 => SpatialIdentifier::Vector,
            1.0 => SpatialIdentifier::Point,
            _ => SpatialIdentifier::Invalid,
        }
    }
}

impl ops::Div<f64> for &SpatialIdentifier {
    type Output = SpatialIdentifier;

    fn div(self, rhs: f64) -> Self::Output {
        match (self.value() as f64) / rhs {
            0.0 => SpatialIdentifier::Vector,
            1.0 => SpatialIdentifier::Point,
            _ => SpatialIdentifier::Invalid,
        }
    }
}

impl<T> From<T> for SpatialIdentifier
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
    use super::SpatialIdentifier;

    #[test]
    fn test_value_getter() {
        // Check that the value method works as expected
        assert_eq!(SpatialIdentifier::Point.value(), 1);
        assert_eq!(SpatialIdentifier::Vector.value(), 0);
        assert_ne!(SpatialIdentifier::Invalid.value(), 1);
        assert_ne!(SpatialIdentifier::Invalid.value(), 0);
    }

    #[test]
    fn test_add() {
        let point = SpatialIdentifier::Point;
        let vector = SpatialIdentifier::Vector;

        assert_eq!(&point + &point, SpatialIdentifier::Invalid);
        assert_eq!(&point + &vector, SpatialIdentifier::Point);
        assert_eq!(&vector + &vector, SpatialIdentifier::Vector);
    }

    #[test]
    fn test_sub() {
        let point = SpatialIdentifier::Point;
        let vector = SpatialIdentifier::Vector;

        assert_eq!(&point - &point, SpatialIdentifier::Vector);
        assert_eq!(&point - &vector, SpatialIdentifier::Point);
        assert_eq!(&vector - &vector, SpatialIdentifier::Vector);
        assert_eq!(&vector - &point, SpatialIdentifier::Invalid);
    }

    #[test]
    fn test_mul() {
        let vector = SpatialIdentifier::Vector;
        let one: f64 = 1.0;
        let zero: f64 = 1.0;

        assert_eq!(&vector * one, SpatialIdentifier::Vector);
        assert_eq!(&vector * zero, SpatialIdentifier::Vector);
    }
}
