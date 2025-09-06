use crate::{
    intersections::{Intersection, Ray},
    lights::Material,
    matrix::{Matrix, Transformable},
    shapes::ShapeBuildable,
    spatial::Tuple,
    utils::EPSILON,
};
use anyhow::Result;
use uuid::Uuid;

use super::{Intersect, Shape, SurfaceNormal};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Representation of a plane in `xz`, extending infinitely
/// far in the `x` and `z` dimensions, and passing through
/// the origin
pub struct Plane {
    _id: Uuid,
    pub transform_matrix: Matrix<4, 4>,
    pub material: Material,
}

impl Plane {
    /// Create a new Plane with the specified transform and material
    pub fn new(transform: Matrix<4, 4>, material: Material) -> Self {
        Self {
            _id: Uuid::new_v4(),
            transform_matrix: transform,
            material,
        }
    }
}

impl Transformable for Plane {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix;
    }
}

impl SurfaceNormal for Plane {
    fn local_normal_at(&self, _point: &Tuple) -> Result<Tuple> {
        Ok(Tuple::vector(0, 1, 0))
    }
}

impl ShapeBuildable for Plane {
    type Built = Plane;

    fn with_material(self, material: Material) -> Self::Built {
        Self {
            _id: self._id,
            transform_matrix: self.transform_matrix,
            material,
        }
    }

    fn with_transform(self, transform: Matrix<4, 4>) -> Self::Built {
        Self {
            _id: self._id,
            transform_matrix: transform,
            material: self.material,
        }
    }
}

impl Intersect for Plane {
    fn local_intersect(&self, ray: &Ray) -> Result<Vec<Intersection>> {
        if ray.direction.get_y().abs() < EPSILON {
            return Ok(vec![]);
        }

        let t = -ray.origin.get_y() / ray.direction.get_y();

        Ok(vec![Intersection::new(t, Shape::Plane(*self))])
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            _id: Uuid::new_v4(),
            transform_matrix: Matrix::<4, 4>::identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Plane;
    use crate::{
        intersections::Ray,
        shapes::{Intersect, Shape, SurfaceNormal},
        spatial::Tuple,
    };
    use anyhow::Result;

    #[test]
    fn normal_of_a_plane_is_constant_everywhere() -> Result<()> {
        let plane = Plane::default();
        let expected_normal = Tuple::vector(0, 1, 0);

        assert_eq!(
            plane.local_normal_at(&Tuple::point(0, 0, 0))?,
            expected_normal
        );

        assert_eq!(
            plane.local_normal_at(&Tuple::point(10, 0, -10))?,
            expected_normal
        );

        assert_eq!(
            plane.local_normal_at(&Tuple::point(-5, 0, -150))?,
            expected_normal
        );

        Ok(())
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() -> Result<()> {
        let p = Plane::default();
        let ray = Ray::new(Tuple::point(0, 10, 0), Tuple::vector(0, 0, 1))?;

        let xs = p.local_intersect(&ray)?;
        assert_eq!(xs.len(), 0);

        Ok(())
    }

    #[test]
    fn intersect_with_coplanar_ray() -> Result<()> {
        let p = Plane::default();
        let ray = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1))?;

        let xs = p.local_intersect(&ray)?;
        assert_eq!(xs.len(), 0);

        Ok(())
    }

    #[test]
    fn ray_intersecting_plane_from_above() -> Result<()> {
        let p = Plane::default();
        let ray = Ray::new(Tuple::point(0, -1, 0), Tuple::vector(0, 1, 0))?;

        let xs = p.local_intersect(&ray)?;
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shape::Plane(p));

        Ok(())
    }

    #[test]
    fn ray_intersecting_plane_from_below() -> Result<()> {
        let p = Plane::default();
        let ray = Ray::new(Tuple::point(0, -1, 0), Tuple::vector(0, 1, 0))?;

        let xs = p.local_intersect(&ray)?;
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, Shape::Plane(p));

        Ok(())
    }
}
