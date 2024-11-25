use crate::intersections::{transform_ray, Intersection, Ray};
use crate::lights::Material;
use crate::matrix::{inverse_4x4, Matrix};
use crate::shapes::Shape;
use crate::spatial::Tuple;
use uuid::Uuid;

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

    /// Calculates the points of intersection for given [Ray] with
    /// the Sphere.
    ///
    /// If there are no points of intersection, an empty vector will
    /// be returned. If there is a tangential intersection, the same
    /// point will be returned twice.
    pub fn intersect(&self, ray: &Ray) -> anyhow::Result<Vec<Intersection>> {
        // First we transform the ray with the inverse of the object's transformation matrix
        // so we can move/deform the ray instead of moving/deforming the object.
        //
        // This enables us to keep the calculation simple since we can assume our unit object
        // centered at the origin (0, 0, 0), and the ray is transformed in relation to it.
        let transformed_ray = transform_ray(ray, &inverse_4x4(&self.transform_matrix)?)?;

        let sphere_to_ray = &transformed_ray.origin - &Tuple::point(0, 0, 0);
        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
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

    pub fn normal_at(&self, point: Tuple) -> anyhow::Result<Tuple> {
        let object_point = &(inverse_4x4(&self.transform_matrix)?) * &point;
        let object_normal = &object_point - &Tuple::point(0, 0, 0);
        let world_normal = &(inverse_4x4(&self.transform_matrix)?.transpose()) * &object_normal;
        Ok(world_normal.convert_to_vector().normalize())
    }

    /// Modify the transform of the sphere
    pub fn set_transform(&mut self, t: Matrix<4, 4>) {
        self.transform_matrix = t;
    }

    /// Set the material for the sphere
    pub fn set_material(&mut self, m: Material) {
        self.material = m;
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
        color::Color,
        intersections::{Computations, Intersection, Ray},
        lights::Material,
        matrix::{rotation_z, scaling, translation, Matrix},
        shapes::{Shape, Sphere},
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
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::default();
        assert_eq!(s.transform_matrix, Matrix::<4, 4>::identity());

        let t = translation(2, 3, 4);
        s.set_transform(t);
        assert_eq!(s.transform_matrix, t);
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
        let mut s = Sphere::default();

        s.set_transform(scaling(2, 2, 2));
        let xs = s.intersect(&r)?;

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);

        Ok(())
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() -> Result<()> {
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let mut s = Sphere::default();

        s.set_transform(translation(5, 0, 0));
        let xs = s.intersect(&r)?;

        assert_eq!(xs.len(), 0);

        Ok(())
    }

    #[test]
    fn surface_normal_for_sphere() -> Result<()> {
        let mut s = Sphere::default();

        // test some basic surface normals out for a unit sphere
        assert_eq!(s.normal_at(Tuple::point(1, 0, 0))?, Tuple::vector(1, 0, 0));
        assert_eq!(s.normal_at(Tuple::point(0, 1, 0))?, Tuple::vector(0, 1, 0));
        assert_eq!(s.normal_at(Tuple::point(0, 0, 1))?, Tuple::vector(0, 0, 1));

        // now we check that the normal vector returned is also normalized
        let n = Tuple::vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );

        assert_eq!(
            s.normal_at(Tuple::point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            ))?,
            n
        );

        assert_eq!(n, n.normalize());

        // the normal_at function should be able to handle transforms
        s.set_transform(translation(0, 1, 0));
        assert_eq!(
            s.normal_at(Tuple::point(0, 1.70711, -FRAC_1_SQRT_2))?,
            Tuple::vector(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );

        let transform = (&scaling(1, 0.5, 1) * &rotation_z(PI / 5.0))?;
        s.set_transform(transform);
        assert_eq!(
            s.normal_at(Tuple::point(0, SQRT_2 / 2.0, -SQRT_2 / 2.0))?,
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
    fn sphere_material_can_be_set() {
        let mut s = Sphere::default();
        let mut m = Material::default();

        m.set_color(Color::green());
        m.set_ambient(0.5);

        s.set_material(m);
        assert_eq!(s.material.get_color(), Color::green());
        assert_eq!(s.material.get_ambient(), 0.5);
    }

    #[test]
    fn the_hit_should_offset_the_point() -> Result<()> {
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let mut shape = Sphere::default();
        shape.set_transform(translation(0, 0, 1));

        let i = Intersection::new(5, Shape::Sphere(shape));
        let comps = Computations::prepare_computations(&i, &r)?;

        assert!(comps.get_over_point().get_z() < -EPSILON / 2.0);
        assert!(comps.get_point().get_z() > comps.get_over_point().get_z());

        Ok(())
    }
}
