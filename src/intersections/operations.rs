use super::{Intersection, Ray};
use crate::matrix::Matrix;
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

pub fn transform_ray(ray: &Ray, matrix: &Matrix<4, 4>) -> Result<Ray> {
    Ray::new(matrix * &ray.origin, matrix * &ray.direction)
}

#[cfg(test)]
mod tests {
    use super::{hit, transform_ray};
    use crate::{
        intersections::{Intersection, Object, Ray, Sphere},
        matrix::{scaling, translation},
        spatial::Tuple,
    };
    use anyhow::Result;

    #[test]
    fn hits_when_all_intersections_have_positive_t() -> Result<()> {
        let s = Sphere::new();
        let i1 = Intersection::new(1, Object::Sphere(s));
        let i2 = Intersection::new(2, Object::Sphere(s));

        assert_eq!(hit(vec![i2, i1]), Some(i1));
        Ok(())
    }

    #[test]
    fn hits_when_some_intersections_have_negative_t() -> Result<()> {
        let s = Sphere::new();
        let i1 = Intersection::new(-1, Object::Sphere(s));
        let i2 = Intersection::new(1, Object::Sphere(s));

        assert_eq!(hit(vec![i2, i1]), Some(i2));

        Ok(())
    }

    #[test]
    fn hits_when_all_intersections_have_negative_t() -> Result<()> {
        let s = Sphere::new();
        let i1 = Intersection::new(-2, Object::Sphere(s));
        let i2 = Intersection::new(-1, Object::Sphere(s));

        assert_eq!(hit(vec![i2, i1]), None);
        Ok(())
    }

    #[test]
    fn hit_is_always_lowest_non_negative_intersection() -> Result<()> {
        let s = Sphere::new();
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
}
