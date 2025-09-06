mod operations;
mod ray;

pub use operations::{calculate_n1_n2, hit, reflect, transform_ray};
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
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl Computations {
    /// Builds a state of the world based on the given intersection and ray
    /// values. This computation is performed to make some commonly accessed
    /// state values easily accessible in other computations.
    pub fn prepare(x: &Intersection, r: &Ray, xs: &[Intersection]) -> Result<Self> {
        // Copy intersection's properties for convenience
        let t = x.t;
        let object = x.object;

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
        let under_point = point - (&normalv * EPSILON);

        let reflectv = reflect(&r.direction, &normalv);

        let n_vals = calculate_n1_n2(xs, x);

        Ok(Self {
            t,
            object,
            point,
            eyev,
            normalv,
            reflectv,
            inside,
            over_point,
            under_point,
            n1: n_vals.0,
            n2: n_vals.1,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::{
        lights::Material,
        matrix::{scaling, translation, Transformable},
        shapes::Plane,
        spatial::Tuple,
        utils::EPSILON,
    };

    use super::{Computations, Intersection, Ray, Shape};
    use crate::shapes::Sphere;
    use anyhow::Result;

    #[test]
    fn precomputing_state_of_intersection_when_it_is_outside() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let sphere = Sphere::default();
        let intersection = Intersection::new(4, Shape::Sphere(sphere));

        let comps = Computations::prepare(&intersection, &ray, &[])?;

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

        let comps = Computations::prepare(&intersection, &ray, &[])?;

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
        let comps = Computations::prepare(&i, &r, &[])?;
        assert_eq!(comps.reflectv, Tuple::vector(0, SQRT_2 / 2.0, SQRT_2 / 2.0));

        Ok(())
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() -> Result<()> {
        // Setup scene and rays
        let mut a = Shape::Sphere(Sphere::glass());
        let material_a = Material {
            refractive_index: 1.5,
            ..Default::default()
        };
        a.set_transform(scaling(2, 2, 2));
        a.set_material(material_a);

        let mut b = Shape::Sphere(Sphere::glass());
        let material_b = Material {
            refractive_index: 2.0,
            ..Default::default()
        };
        b.set_transform(translation(0, 0, -0.25));
        b.set_material(material_b);

        let mut c = Shape::Sphere(Sphere::glass());
        let material_c = Material {
            refractive_index: 2.5,
            ..Default::default()
        };
        b.set_transform(translation(0, 0, 0.25));
        c.set_material(material_c);

        let ray = Ray::new(Tuple::point(0, 0, -4), Tuple::vector(0, 0, 1))?;
        let xs = vec![
            Intersection::new(2, a),
            Intersection::new(2.75, b),
            Intersection::new(3.25, c),
            Intersection::new(4.75, b),
            Intersection::new(5.25, c),
            Intersection::new(6, a),
        ];

        let expectations = [
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for i in 0..5 {
            let comps = Computations::prepare(&xs[i], &ray, &xs)?;
            assert_eq!(comps.n1, expectations[i].0);
            assert_eq!(comps.n2, expectations[i].1);
        }

        Ok(())
    }

    #[test]
    fn under_point_is_offset_below_surface() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;

        let mut shape = Shape::Sphere(Sphere::glass());
        shape.set_transform(translation(0, 0, 1));

        let i = Intersection::new(5, shape);
        let xs = vec![i];

        let comps = Computations::prepare(&i, &ray, &xs)?;

        assert!(comps.under_point.get_z() > (EPSILON / 2.0));
        assert!(comps.point.get_z() < comps.under_point.get_z());

        Ok(())
    }
}
