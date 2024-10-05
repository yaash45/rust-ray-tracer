use super::{transform_ray, Ray};
use crate::{
    matrix::{inverse_4x4, Matrix},
    spatial::Tuple,
};
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Stores all the variants of the Object type
pub enum Object {
    Sphere(Sphere),
}

#[derive(Debug, Clone, Copy)]
/// Representation of a unit sphere centred at (0,0,0)
pub struct Sphere {
    /// Added this field so that no two invocations of the
    /// default / new will return the same Sphere. We want
    /// to maintain uniqueness with each creation.
    _id: Uuid,
    transform_matrix: Matrix<4, 4>,
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
    pub fn intersect(&self, ray: &Ray) -> Result<Vec<Intersection>> {
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

            let i1 = Intersection::new(t1, Object::Sphere(*self));
            let i2 = Intersection::new(t2, Object::Sphere(*self));

            Ok(vec![i1, i2])
        }
    }

    /// Modify the transform of a sphere
    pub fn set_transform(&mut self, t: Matrix<4, 4>) {
        self.transform_matrix = t;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            _id: Uuid::new_v4(),
            transform_matrix: Matrix::<4, 4>::identity(),
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// Data structure to keep track of intersections
/// for a given object
pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

impl Intersection {
    /// Create a new Intersection for a given object using
    /// the calculated `t` value of a Ray intersecting `object`
    pub fn new(t: impl Into<f64>, object: Object) -> Self {
        Self {
            t: t.into(),
            object,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Intersection, Object, Ray, Sphere};
    use crate::{
        matrix::{scaling, translation, Matrix},
        spatial::Tuple,
    };
    use anyhow::Result;

    #[test]
    fn create_a_default_sphere() {
        let s = Sphere::new();
        assert_eq!(s.transform_matrix, Matrix::<4, 4>::identity());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        assert_eq!(s.transform_matrix, Matrix::<4, 4>::identity());

        let t = translation(2, 3, 4);
        s.set_transform(t);
        assert_eq!(s.transform_matrix, t);
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[0].object, Object::Sphere(s));
        assert_eq!(xs[1].t, 6.0);
        assert_eq!(xs[1].object, Object::Sphere(s));
        Ok(())
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 1, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[0].object, Object::Sphere(s));
        assert_eq!(xs[1].t, 5.0);
        assert_eq!(xs[1].object, Object::Sphere(s));
        Ok(())
    }

    #[test]
    fn ray_misses_a_sphere() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 2, -5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 0);
        Ok(())
    }

    #[test]
    fn ray_originates_inside_sphere() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[0].object, Object::Sphere(s));
        assert_eq!(xs[1].t, 1.0);
        assert_eq!(xs[1].object, Object::Sphere(s));
        Ok(())
    }

    #[test]
    fn sphere_is_behind_a_ray() -> Result<()> {
        let ray = Ray::new(Tuple::point(0, 0, 5), Tuple::vector(0, 0, 1))?;
        let s = Sphere::new();

        let xs = s.intersect(&ray)?;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[0].object, Object::Sphere(s));
        assert_eq!(xs[1].t, -4.0);
        assert_eq!(xs[1].object, Object::Sphere(s));
        Ok(())
    }

    #[test]
    fn creating_intersection_works() {
        let s = Sphere::new();
        let t = 3.5;
        let i = Intersection::new(t, Object::Sphere(s));
        assert_eq!(i.t, t);
        assert_eq!(i.object, Object::Sphere(s));
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() -> Result<()> {
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let mut s = Sphere::new();

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
        let mut s = Sphere::new();

        s.set_transform(translation(5, 0, 0));
        let xs = s.intersect(&r)?;

        assert_eq!(xs.len(), 0);

        Ok(())
    }
}
