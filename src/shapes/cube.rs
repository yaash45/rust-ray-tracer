use crate::intersections::{Intersection, Ray};
use crate::lights::Material;
use crate::matrix::{Matrix, Transformable};
use crate::shapes::{Intersect, Shape, ShapeBuildable, SurfaceNormal};
use crate::spatial::Tuple;
use crate::utils::EPSILON;
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Cube {
    _id: Uuid,
    pub transform_matrix: Matrix<4, 4>,
    pub material: Material,
}

impl Cube {
    pub fn new(transform_matrix: Matrix<4, 4>, material: Material) -> Self {
        Self {
            _id: Uuid::new_v4(),
            transform_matrix,
            material,
        }
    }
}

impl Transformable for Cube {
    fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform_matrix
    }

    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>) {
        self.transform_matrix = transform_matrix;
    }
}

impl SurfaceNormal for Cube {
    fn local_normal_at(&self, point: &Tuple) -> Result<Tuple> {
        let (abs_x, abs_y, abs_z) = (
            point.get_x().abs(),
            point.get_y().abs(),
            point.get_z().abs(),
        );

        let max_c = abs_x.max(abs_y).max(abs_z);

        let normal = if max_c == abs_x {
            Tuple::vector(point.get_x(), 0.0, 0.0)
        } else if max_c == abs_y {
            Tuple::vector(0.0, point.get_y(), 0.0)
        } else {
            Tuple::vector(0.0, 0.0, point.get_z())
        };

        Ok(normal)
    }
}

impl ShapeBuildable for Cube {
    type Built = Cube;

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

impl Intersect for Cube {
    fn local_intersect(&self, transformed_ray: &Ray) -> anyhow::Result<Vec<Intersection>> {
        let (xmin, xmax) = check_axis(
            transformed_ray.origin.get_x(),
            transformed_ray.direction.get_x(),
        );

        let (ymin, ymax) = check_axis(
            transformed_ray.origin.get_y(),
            transformed_ray.direction.get_y(),
        );

        let (zmin, zmax) = check_axis(
            transformed_ray.origin.get_z(),
            transformed_ray.direction.get_z(),
        );

        let tmin = xmin.max(ymin).max(zmin);
        let tmax = xmax.min(ymax).min(zmax);

        if tmin > tmax {
            return Ok(vec![]);
        }

        Ok(vec![
            Intersection::new(tmin, Shape::Cube(*self)),
            Intersection::new(tmax, Shape::Cube(*self)),
        ])
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1_f64 - origin;
    let tmax_numerator = 1_f64 - origin;

    let (mut tmin, mut tmax) = match direction.abs().total_cmp(&EPSILON) {
        std::cmp::Ordering::Less => (
            tmin_numerator * f64::INFINITY,
            tmax_numerator * f64::INFINITY,
        ),
        _ => (tmin_numerator / direction, tmax_numerator / direction),
    };

    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }

    (tmin, tmax)
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            _id: Uuid::new_v4(),
            transform_matrix: Matrix::<4, 4>::identity(),
            material: Material::default(),
        }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
}

#[cfg(test)]
mod tests {
    use crate::intersections::Ray;
    use crate::shapes::cube::Cube;
    use crate::shapes::{Intersect, SurfaceNormal};
    use crate::spatial::Tuple;
    use anyhow::Result;

    #[test]
    fn ray_intersects_a_cube() -> Result<()> {
        let cube = Cube::default();

        let cases = [
            ((5.0, 0.5, 0.0), (-1.0, 0.0, 0.0), 2, 4.0, 6.0),
            ((-5.0, 0.5, 0.0), (1.0, 0.0, 0.0), 2, 4.0, 6.0),
            ((0.5, 5.0, 0.0), (0.0, -1.0, 0.0), 2, 4.0, 6.0),
            ((0.5, -5.0, 0.0), (0.0, 1.0, 0.0), 2, 4.0, 6.0),
            ((0.5, 0.0, 5.0), (0.0, 0.0, -1.0), 2, 4.0, 6.0),
            ((0.5, 0.0, -5.0), (0.0, 0.0, 1.0), 2, 4.0, 6.0),
            ((0.0, 0.5, 0.0), (0.0, 0.0, 1.0), 2, -1.0, 1.0),
        ];

        for (origin, direction, expected_count, expected_t1, expected_t2) in cases {
            let ray = Ray::new(
                Tuple::point(origin.0, origin.1, origin.2),
                Tuple::vector(direction.0, direction.1, direction.2),
            )?;

            let intersections = cube.local_intersect(&ray)?;

            assert_eq!(intersections.len(), expected_count);
            assert_eq!(intersections[0].t, expected_t1);
            assert_eq!(intersections[1].t, expected_t2);
        }

        Ok(())
    }

    #[test]
    fn ray_misses_a_cube() -> Result<()> {
        let cube = Cube::default();

        let cases = [
            ((-2.0, 0.0, 0.0), (0.2673, 0.5345, 0.8018)),
            ((0.0, -2.0, 0.0), (0.8018, 0.2673, 0.5345)),
            ((0.0, 0.0, -2.0), (0.5345, 0.8018, 0.2673)),
            ((2.0, 0.0, 2.0), (0.0, 0.0, -1.0)),
            ((0.0, 2.0, 2.0), (0.0, -1.0, 0.0)),
            ((2.0, 2.0, 0.0), (-1.0, 0.0, 0.0)),
        ];

        for (origin, direction) in cases {
            let ray = Ray::new(
                Tuple::point(origin.0, origin.1, origin.2),
                Tuple::vector(direction.0, direction.1, direction.2),
            )?;

            let intersections = cube.local_intersect(&ray)?;

            assert_eq!(intersections.len(), 0);
        }

        Ok(())
    }

    #[test]
    fn surface_normal_for_cube() -> Result<()> {
        let cube = Cube::default();

        let cases = [
            ((1.0, 0.5, -0.8), (1, 0, 0)),
            ((-1.0, -0.2, 0.9), (-1, 0, 0)),
            ((-0.4, 1.0, -0.1), (0, 1, 0)),
            ((0.3, -1.0, -0.7), (0, -1, 0)),
            ((-0.6, 0.3, 1.0), (0, 0, 1)),
            ((0.4, 0.4, -1.0), (0, 0, -1)),
            ((1.0, 1.0, 1.0), (1, 0, 0)),
            ((-1.0, -1.0, -1.0), (-1, 0, 0)),
        ];

        for (point, expected) in cases {
            let p = Tuple::point(point.0, point.1, point.2);
            let normal = cube.local_normal_at(&p)?;
            assert_eq!(normal, Tuple::vector(expected.0, expected.1, expected.2));
        }

        Ok(())
    }
}
