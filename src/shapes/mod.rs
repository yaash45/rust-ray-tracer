mod sphere;

pub use sphere::Sphere;

use {
    crate::{
        intersections::{transform_ray, Intersection, Ray},
        lights::Material,
        matrix::{inverse_4x4, Matrix},
        spatial::Tuple,
    },
    anyhow::Result,
};

/// Trait that can be used to implement a way to get
/// surface normals for any Shapes that might implement
/// this trait
pub trait SurfaceNormal {
    /// Returns a normalized surface normal vector for
    /// any Shape that implements this method
    fn normal_at(&self, point: Tuple) -> Result<Tuple>;
}

/// Trait that can be used to implement an intersection
/// calculation for any Shapes that implement it
pub trait Intersect {
    /// Calculates the points of intersection for given [Ray] with
    /// the Shape implementing this trait.
    ///
    /// If there are no points of intersection, an empty vector will
    /// be returned. If there is a tangential intersection, the same
    /// point will be returned twice.
    fn intersect(&self, ray: &Ray) -> Result<Vec<Intersection>> {
        // First we transform the ray with the inverse of the object's transformation matrix
        // so we can move/deform the ray instead of moving/deforming the object.
        //
        // This enables us to keep the calculation simple since we can assume our unit object
        // centered at the origin (0, 0, 0), and the ray is transformed in relation to it.
        let transformed_ray = transform_ray(ray, &inverse_4x4(&self.get_transform())?)?;
        self.local_intersect(&transformed_ray)
    }

    /// Returns the object's transformation matrix
    fn get_transform(&self) -> Matrix<4, 4>;

    /// Returns the local intersection points of the Shape
    fn local_intersect(&self, transformed_ray: &Ray) -> Result<Vec<Intersection>>;
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Stores all the variants of the Shape type
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    /// Get the material of the Shape
    pub fn get_material(&self) -> Material {
        match self {
            Shape::Sphere(ref sphere) => sphere.material,
        }
    }

    /// Set the material of the Shape
    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(ref mut sphere) => sphere.material = material,
        }
    }

    /// Get the transform matrix of the Shape
    pub fn get_transform(&self) -> Matrix<4, 4> {
        match self {
            Shape::Sphere(ref sphere) => sphere.transform_matrix,
        }
    }

    /// Set the transform matrix of the Shape
    pub fn set_transform(&mut self, transform: Matrix<4, 4>) {
        match self {
            Shape::Sphere(ref mut sphere) => sphere.transform_matrix = transform,
        }
    }
}

impl SurfaceNormal for Shape {
    fn normal_at(&self, point: Tuple) -> anyhow::Result<Tuple> {
        match self {
            Shape::Sphere(ref sphere) => sphere.normal_at(point),
        }
    }
}

impl Intersect for Shape {
    fn get_transform(&self) -> Matrix<4, 4> {
        self.get_transform()
    }

    fn local_intersect(&self, transformed_ray: &Ray) -> Result<Vec<Intersection>> {
        match self {
            Shape::Sphere(ref sphere) => sphere.local_intersect(transformed_ray),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Shape, Sphere};
    use crate::lights::Material;
    use crate::matrix::{translation, Matrix};

    struct ShapeFactory {}

    impl ShapeFactory {
        /// Create a default shape to test against
        fn test_shape() -> Shape {
            Shape::Sphere(Sphere::default())
        }
    }

    #[test]
    fn shape_transformations() {
        let mut s = ShapeFactory::test_shape();

        // by default, the transform matrix of any shape must be an identity matrix
        assert_eq!(s.get_transform(), Matrix::identity());

        // the transform matrix of any shape can be set
        let translation_matrix = translation(2, 3, 5);
        s.set_transform(translation_matrix);
        assert_eq!(s.get_transform(), translation_matrix);
    }

    #[test]
    fn shape_materials() {
        let mut s = ShapeFactory::test_shape();

        // by default, the material of any shape must be the default material
        assert_eq!(s.get_material(), Material::default());

        // the material of any shape can be set
        let mut new_material = Material::default();
        new_material.set_ambient(0.5);
        s.set_material(new_material);
        assert_eq!(s.get_material(), new_material);
    }
}
