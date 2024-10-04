use crate::spatial::Tuple;
use anyhow::{Error, Result};
use uuid::Uuid;

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

/// Representation of a unit sphere centred at (0,0,0)
pub struct Sphere {
    /// Added this field so that no two invocations of the
    /// default / new will return the same Sphere. We want
    /// to maintain uniqueness with each creation.
    _id: Uuid,
}

impl Sphere {
    /// Create a new [Sphere]
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculates the `t` values for the points of intersection of
    /// a given [Ray] with this [Sphere].
    ///
    /// If there are no points of intersection, an empty vector will
    /// be returned. If there is a tangential intersection, the same
    /// point will be returned twice.
    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = &ray.origin - &Tuple::point(0, 0, 0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - (4.0 * a * c);

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            _id: Uuid::new_v4(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Ray, Sphere};
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

    #[test]
    fn ray_intersects_sphere_at_two_points() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
        Ok(())
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 1, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
        Ok(())
    }

    #[test]
    fn ray_misses_a_sphere() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 2, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray);
        assert_eq!(xs.len(), 0);
        Ok(())
    }

    #[test]
    fn ray_originates_inside_sphere() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
        Ok(())
    }

    #[test]
    fn sphere_is_behind_a_ray() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, 5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
        Ok(())
    }
}
