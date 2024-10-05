use crate::spatial::Tuple;
use anyhow::{Error, Result};

#[derive(Debug, Copy, Clone)]
/// A data structure representing the origin and direction of a ray
pub struct Ray {
    /// The origin point of this [Ray]
    pub origin: Tuple,
    /// The direction of this [Ray]
    pub direction: Tuple,
}

impl Ray {
    /// We need to ensure that the users pass in valid origin
    /// and direction values to create new [Rays]. We want to maintain
    /// the invariant that the origin is always a point, and that a
    /// direction is always a vector.
    fn validate(origin: &Tuple, direction: &Tuple) -> bool {
        origin.is_a_point() && direction.is_a_vector()
    }

    /// Given a starting origin point, and a direction vector,
    /// we can create a new [Ray] using this constructor
    pub fn new(origin: Tuple, direction: Tuple) -> Result<Self> {
        if !Self::validate(&origin, &direction) {
            return Err(Error::msg(
                "The origin tuple must be a point, and the direction tuple must be a vector",
            ));
        }
        Ok(Self { origin, direction })
    }

    /// Finds the point `t` units away in the direction of this
    /// [Ray] from the origin of this [Ray]
    pub fn position(&self, t: impl Into<f64>) -> Tuple {
        self.origin + (&self.direction * t.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::spatial::Tuple;
    use anyhow::Result;

    #[test]
    fn creation_and_querying() -> Result<()> {
        let origin = Tuple::point(1, 2, 3);
        let direction = Tuple::vector(4, 5, 6);
        let ray = Ray::new(origin, direction)?;
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
        Ok(())
    }

    #[test]
    fn calculate_position() -> Result<()> {
        let ray = Ray::new(Tuple::point(2, 3, 4), Tuple::vector(1, 0, 0))?;

        assert_eq!(ray.position(0), ray.origin);
        assert_eq!(ray.position(1), Tuple::point(3, 3, 4));
        assert_eq!(ray.position(-1), Tuple::point(1, 3, 4));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3, 4));
        Ok(())
    }
}
