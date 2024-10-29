use super::{Intersection, Ray};
use crate::{matrix::Matrix, spatial::Tuple};
use anyhow::Result;
use core::f64;

/// Given a set of intersections, this function returns the one
/// that has the lowest non negative t-value. This corresponds to
/// the intersection that hits an object.
///
/// In the event that the ray misses the object entirely,
/// this would return a `None` value.
pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    let mut result: Option<Intersection> = None;
    let mut current_min = f64::MAX;

    for i in xs {
        // find the lowest non-negative t value to find
        // the ray that hit the object
        if i.t < current_min && i.t > 0.0 {
            current_min = i.t;
            result = Some(i);
        }
    }

    result
}

/// Transforms a ray by performing a matrix multiplication
/// of the ray and the given input matrix. This is useful
/// to transform rays instead of transforming objects themselves.
pub fn transform_ray(ray: &Ray, matrix: &Matrix<4, 4>) -> Result<Ray> {
    Ray::new(matrix * &ray.origin, matrix * &ray.direction)
}

/// Calculates the reflection of an inbound vector for a
/// surface given the normal vector for that point.
pub fn reflect(inbound: &Tuple, normal: &Tuple) -> Tuple {
    inbound - &(normal * (2.0 * normal.dot(inbound)))
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::{hit, reflect, transform_ray, Intersection};
    use crate::{
        intersections::{Object, Ray, Sphere},
        matrix::{scaling, translation},
        spatial::Tuple,
    };
    use anyhow::Result;

    #[test]
    fn hits_when_all_intersections_have_positive_t() -> Result<()> {
        let s = Sphere::default();
        let i1 = Intersection::new(1, Object::Sphere(s));
        let i2 = Intersection::new(2, Object::Sphere(s));

        assert_eq!(hit(vec![i2, i1]), Some(i1));
        Ok(())
    }

    #[test]
    fn hits_when_some_intersections_have_negative_t() -> Result<()> {
        let s = Sphere::default();
        let i1 = Intersection::new(-1, Object::Sphere(s));
        let i2 = Intersection::new(1, Object::Sphere(s));

        assert_eq!(hit(vec![i2, i1]), Some(i2));

        Ok(())
    }

    #[test]
    fn hits_when_all_intersections_have_negative_t() -> Result<()> {
        let s = Sphere::default();
        let i1 = Intersection::new(-2, Object::Sphere(s));
        let i2 = Intersection::new(-1, Object::Sphere(s));

        assert_eq!(hit(vec![i2, i1]), None);
        Ok(())
    }

    #[test]
    fn hit_is_always_lowest_non_negative_intersection() -> Result<()> {
        let s = Sphere::default();
        let i1 = Intersection::new(5, Object::Sphere(s));
        let i2 = Intersection::new(7, Object::Sphere(s));
        let i3 = Intersection::new(-3, Object::Sphere(s));
        let i4 = Intersection::new(2, Object::Sphere(s));

        assert_eq!(hit(vec![i1, i2, i3, i4]), Some(i4));

        Ok(())
    }

    #[test]
    fn translating_a_ray() -> Result<()> {
        let r = Ray::new(Tuple::point(1, 2, 3), Tuple::vector(0, 1, 0))?;
        let m = translation(3, 4, 5);
        let r2 = transform_ray(&r, &m)?;
        assert_eq!(r2.origin, Tuple::point(4, 6, 8));
        assert_eq!(r2.direction, Tuple::vector(0, 1, 0));
        Ok(())
    }

    #[test]
    fn scaling_a_ray() -> Result<()> {
        let r = Ray::new(Tuple::point(1, 2, 3), Tuple::vector(0, 1, 0))?;
        let m = scaling(2, 3, 4);
        let r2 = transform_ray(&r, &m)?;
        assert_eq!(r2.origin, Tuple::point(2, 6, 12));
        assert_eq!(r2.direction, Tuple::vector(0, 3, 0));
        Ok(())
    }

    #[test]
    fn reflect_operation_works() {
        // Reflecting a vector approaching at 45 degree angle
        let v = Tuple::vector(1, -1, 0);
        let n = Tuple::vector(0, 1, 0);
        assert_eq!(reflect(&v, &n), Tuple::vector(1, 1, 0));

        // Reflecting off a slanted surface
        let v = Tuple::vector(0, -1, 0);
        let n = Tuple::vector(SQRT_2 / 2.0, SQRT_2 / 2.0, 0);
        assert_eq!(reflect(&v, &n), Tuple::vector(1, 0, 0));
    }
}
