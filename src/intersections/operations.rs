use super::Intersection;
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

#[cfg(test)]
mod tests {
    use super::hit;
    use crate::intersections::{Intersection, Object, Sphere};
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
}
