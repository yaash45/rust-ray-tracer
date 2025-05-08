mod operations;
mod ray;

pub use operations::{hit, reflect, transform_ray};
pub use ray::Ray;

use crate::{
    shapes::{Shape, SurfaceNormal},
    spatial::Tuple,
    utils::EPSILON,
};
use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
/// Data structure to keep track of intersections
/// for a given object
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl Intersection {
    /// Create a new Intersection for a given object using
    /// the calculated `t` value of a Ray intersecting `object`
    pub fn new(t: impl Into<f64>, object: Shape) -> Self {
        Self {
            t: t.into(),
            object,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Struct containing pre-computed values using rays and intersections
pub struct Computations {
    pub t: f64,
    pub object: Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

impl Computations {
    /// Builds a state of the world based on the given intersection and ray
    /// values. This computation is performed to make some commonly accessed
    /// state values easily accessible in other computations.
    pub fn prepare(i: &Intersection, r: &Ray) -> Result<Self> {
        // Copy intersection's properties for convenience
        let t = i.t;
        let object = i.object;

        // Precompute some useful values
        let point = r.position(t);
        let eyev = -r.direction;
        let mut normalv = object.normal_at(&point)?;
        let mut inside = false;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let over_point = point + (&normalv * EPSILON);

        let reflectv = reflect(&r.direction, &normalv);

        Ok(Self {
            t,
            object,
            point,
            eyev,
            normalv,
            reflectv,
            inside,
            over_point,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::{shapes::Plane, spatial::Tuple};

    use super::{Computations, Intersection, Ray, Shape};
    use crate::shapes::Sphere;
    use anyhow::Result;

    #[test]
    fn precomputing_state_of_intersection_when_it_is_outside() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let sphere = Sphere::default();
        let intersection = Intersection::new(4, Shape::Sphere(sphere));

        let comps = Computations::prepare(&intersection, &ray)?;

        assert_eq!(comps.t, intersection.t);
        assert_eq!(comps.object, Shape::Sphere(sphere));
        assert_eq!(comps.point, Tuple::point(0, 0, -1));
        assert_eq!(comps.eyev, Tuple::vector(0, 0, -1));
        assert_eq!(comps.normalv, Tuple::vector(0, 0, -1));
        assert!(!comps.inside);

        Ok(())
    }

    #[test]
    fn precomputing_state_of_intersection_when_it_is_inside() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1))?;
        let sphere = Sphere::default();
        let intersection = Intersection::new(1, Shape::Sphere(sphere));

        let comps = Computations::prepare(&intersection, &ray)?;

        assert_eq!(comps.t, 1.0);
        assert_eq!(comps.point, Tuple::point(0, 0, 1));
        assert_eq!(comps.eyev, Tuple::vector(0, 0, -1));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Tuple::vector(0, 0, -1));

        Ok(())
    }

    #[test]
    fn precomputing_the_reflection_vector() -> Result<()> {
        let shape = Shape::Plane(Plane::default());
        let r = Ray::new(
            Tuple::point(0, 1, -1),
            Tuple::vector(0, -(SQRT_2 / 2.0), SQRT_2 / 2.0),
        )?;
        let i = Intersection::new(SQRT_2, shape);
        let comps = Computations::prepare(&i, &r)?;
        assert_eq!(comps.reflectv, Tuple::vector(0, SQRT_2 / 2.0, SQRT_2 / 2.0));

        Ok(())
    }
}
