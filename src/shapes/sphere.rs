use {
    super::{Intersect, Shape, SurfaceNormal},
    crate::{
        intersections::{Intersection, Ray},
        lights::Material,
        matrix::Matrix,
        spatial::Tuple,
    },
    anyhow::Result,
    uuid::Uuid,
};

#[derive(Debug, Clone, Copy, PartialOrd)]
/// Representation of a unit sphere centred at (0,0,0)
pub struct Sphere {
    /// Added this field so that no two invocations of the
    /// default / new will return the same Sphere. We want
    /// to maintain uniqueness with each creation.
    _id: Uuid,
    pub transform_matrix: Matrix<4, 4>,
    pub material: Material,
}

impl Sphere {
    /// Create a new [Sphere]
    pub fn new(transform: Matrix<4, 4>, material: Material) -> Self {
        Self {
            _id: Uuid::new_v4(),
            transform_matrix: transform,
            material,
        }
    }
}

impl SurfaceNormal for Sphere {
    fn local_normal_at(&self, point: &Tuple) -> Result<Tuple> {
        Ok(point - &Tuple::point(0, 0, 0))
    }

    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }
}

impl Intersect for Sphere {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn local_intersect(&self, ray: &Ray) -> Result<Vec<Intersection>> {
        let sphere_to_ray = &ray.origin - &Tuple::point(0, 0, 0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - (4.0 * a * c);

        if discriminant < 0.0 {
            Ok(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let i1 = Intersection::new(t1, Shape::Sphere(*self));
            let i2 = Intersection::new(t2, Shape::Sphere(*self));

            Ok(vec![i1, i2])
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            _id: Uuid::new_v4(),
            transform_matrix: Matrix::<4, 4>::identity(),
            material: Material::default(),
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use crate::{
        intersections::{Computations, Intersection, Ray},
        lights::Material,
        matrix::{rotation_z, scaling, translation, Matrix},
        shapes::{Intersect, Shape, Sphere, SurfaceNormal},
        spatial::Tuple,
        utils::EPSILON,
    };
    use anyhow::Result;

    #[test]
    fn create_a_default_sphere() {
        let s = Sphere::default();
        assert_eq!(s.transform_matrix, Matrix::<4, 4>::identity());
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::default();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[0].object, Shape::Sphere(s));
        assert_eq!(xs[1].t, 6.0);
        assert_eq!(xs[1].object, Shape::Sphere(s));
        Ok(())
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 1, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::default();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[0].object, Shape::Sphere(s));
        assert_eq!(xs[1].t, 5.0);
        assert_eq!(xs[1].object, Shape::Sphere(s));
        Ok(())
    }

    #[test]
    fn ray_misses_a_sphere() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 2, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::default();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 0);
        Ok(())
    }

    #[test]
    fn ray_originates_inside_sphere() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1))?;
        let s = Sphere::default();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[0].object, Shape::Sphere(s));
        assert_eq!(xs[1].t, 1.0);
        assert_eq!(xs[1].object, Shape::Sphere(s));
        Ok(())
    }

    #[test]
    fn sphere_is_behind_a_ray() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, 5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::default();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[0].object, Shape::Sphere(s));
        assert_eq!(xs[1].t, -4.0);
        assert_eq!(xs[1].object, Shape::Sphere(s));
        Ok(())
    }

    #[test]
    fn creating_intersection_works() {
        let s = Sphere::default();
        let t = 3.5;
        let i = Intersection::new(t, Shape::Sphere(s));
        assert_eq!(i.t, t);
        assert_eq!(i.object, Shape::Sphere(s));
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() -> Result<()> {
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere {
            transform_matrix: scaling(2, 2, 2),
            ..Default::default()
        };

        let xs = s.intersect(&r)?;

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);

        Ok(())
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() -> Result<()> {
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere {
            transform_matrix: translation(6, 0, 0),
            ..Default::default()
        };

        let xs = s.intersect(&r)?;

        assert_eq!(xs.len(), 0);

        Ok(())
    }

    #[test]
    fn surface_normal_for_sphere() -> Result<()> {
        let mut s = Sphere::default();

        // test some basic surface normals out for a unit sphere
        assert_eq!(s.normal_at(&Tuple::point(1, 0, 0))?, Tuple::vector(1, 0, 0));
        assert_eq!(s.normal_at(&Tuple::point(0, 1, 0))?, Tuple::vector(0, 1, 0));
        assert_eq!(s.normal_at(&Tuple::point(0, 0, 1))?, Tuple::vector(0, 0, 1));

        // now we check that the normal vector returned is also normalized
        let n = Tuple::vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );

        assert_eq!(
            s.normal_at(&Tuple::point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            ))?,
            n
        );

        assert_eq!(n, n.normalize());

        // the normal_at function should be able to handle transforms
        s.transform_matrix = translation(0, 1, 0);
        assert_eq!(
            s.normal_at(&Tuple::point(0, 1.70711, -FRAC_1_SQRT_2))?,
            Tuple::vector(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );

        let transform = (&scaling(1, 0.5, 1) * &rotation_z(PI / 5.0))?;
        s.transform_matrix = transform;
        assert_eq!(
            s.normal_at(&Tuple::point(0, SQRT_2 / 2.0, -SQRT_2 / 2.0))?,
            Tuple::vector(0, 0.97014, -0.24254)
        );

        Ok(())
    }

    #[test]
    fn sphere_starts_with_default_material() {
        let s = Sphere::default();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn the_hit_should_offset_the_point() -> Result<()> {
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let shape = Sphere {
            transform_matrix: translation(1, 0, 1),
            ..Default::default()
        };

        let i = Intersection::new(5, Shape::Sphere(shape));
        let comps = Computations::prepare_computations(&i, &r)?;

        assert!(comps.get_over_point().get_z() < -EPSILON / 2.0);
        assert!(comps.get_point().get_z() > comps.get_over_point().get_z());

        Ok(())
    }
}
