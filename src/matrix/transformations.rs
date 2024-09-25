use crate::matrix::Matrix;

/// Gets a 4x4 transformation matrix that can be used to translate tuples in 3D space
///
/// ```
/// use raytracer::{matrix::translation, spatial::Tuple};
///
/// let translator = translation(5, -3, 2);
/// let point = Tuple::new_point(1, 1, 1);
/// let expected_translated_point = Tuple::new_point(6, -2, 3);
///
/// assert_eq!(&translator * &point, expected_translated_point);
/// ```
pub fn translation(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Matrix<4, 4> {
    Matrix::from([
        [1.0, 0.0, 0.0, x.into()],
        [0.0, 1.0, 0.0, y.into()],
        [0.0, 0.0, 1.0, z.into()],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Gets a 4x4 scaling matrix that can be used to grow/shrink tuples in 3D space
///
/// ```
/// use raytracer::{matrix::scaling, spatial::Tuple};
///
/// let scaling = scaling(2, 2, 2);
/// let point = Tuple::new_point(1, 1, 1);
/// let expected_scaled_point = Tuple::new_point(2, 2, 2);
///
/// assert_eq!(&scaling * &point, expected_scaled_point);
/// ```
pub fn scaling(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Matrix<4, 4> {
    Matrix::from([
        [x.into(), 0.0, 0.0, 0.0],
        [0.0, y.into(), 0.0, 0.0],
        [0.0, 0.0, z.into(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Gets a 4x4 matrix that can rotate a [crate::spatial::Tuple]
/// by `rad` radians around the x-axis
pub fn rotation_x(radians: f64) -> Matrix<4, 4> {
    Matrix::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, radians.cos(), -(radians.sin()), 0.0],
        [0.0, radians.sin(), radians.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Gets a 4x4 matrix that can rotate a [crate::spatial::Tuple]
/// by `rad` radians around the y-axis
pub fn rotation_y(radians: f64) -> Matrix<4, 4> {
    Matrix::from([
        [radians.cos(), 0.0, radians.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-radians.sin(), 0.0, radians.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Gets a 4x4 matrix that can rotate a [crate::spatial::Tuple]
/// by `rad` radians around the z-axis
pub fn rotation_z(rad: f64) -> Matrix<4, 4> {
    Matrix::from([
        [rad.cos(), -rad.sin(), 0.0, 0.0],
        [rad.sin(), rad.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Gets a 4x4 matrix that can apply a shear transformation on
/// [crate::spatial::Tuple]
pub fn shearing(
    x_y: impl Into<f64>,
    x_z: impl Into<f64>,
    y_x: impl Into<f64>,
    y_z: impl Into<f64>,
    z_x: impl Into<f64>,
    z_y: impl Into<f64>,
) -> Matrix<4, 4> {
    Matrix::from([
        [1.0, x_y.into(), x_z.into(), 0.0],
        [y_x.into(), 1.0, y_z.into(), 0.0],
        [z_x.into(), z_y.into(), 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::{rotation_x, rotation_y, rotation_z, scaling, shearing, translation};
    use crate::matrix::inverse_4x4;
    use crate::spatial::Tuple;
    use anyhow::Result;

    #[test]
    fn translation_of_tuples() -> Result<()> {
        let point = Tuple::new_point(-3, 4, 5);
        let transform = translation(5, -3, 2);
        let expected_destination = Tuple::new_point(2, 1, 7);

        // Translating the input point to the expected destination
        assert_eq!(&transform * &point, expected_destination);

        // When using the inverse of the translation matrix, we can
        // go back from the destination to the original point location
        let inv = inverse_4x4(&transform)?;
        assert_eq!(&inv * &expected_destination, point);

        // The translation of a vector is the same vector
        let vector = Tuple::new_vector(-3, 4, 5);
        assert_eq!(&transform * &vector, vector);

        Ok(())
    }

    #[test]
    fn scaling_of_tuples() -> Result<()> {
        let transform = scaling(2, 3, 4);
        let point = Tuple::new_point(-4, 6, 8);
        let vector = Tuple::new_vector(-4, 6, 8);

        // Scaling applied to the point
        let expected_scaled_point = Tuple::new_point(-8, 18, 32);
        assert_eq!(&transform * &point, expected_scaled_point);

        // Scaling applied to a vector
        let expected_scaled_vector = Tuple::new_vector(-8, 18, 32);
        assert_eq!(&transform * &vector, expected_scaled_vector);

        // Scaling by the inverse will shrink the tuple instead of growing it
        let inv = inverse_4x4(&transform)?;
        assert_eq!(&inv * &expected_scaled_point, point);
        assert_eq!(&inv * &expected_scaled_vector, vector);

        // We can use scaling to reflect a point along any axes
        let reflect_x = scaling(-1, 1, 1);
        let reflected_point = Tuple::new_point(-point.get_x(), point.get_y(), point.get_z());
        assert_eq!(&reflect_x * &point, reflected_point);

        Ok(())
    }

    #[test]
    fn rotation_x_tests() -> Result<()> {
        let p = Tuple::new_point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        let expected_half_quarter_point =
            Tuple::new_point(0, ((2_f64).sqrt()) / 2_f64, ((2_f64).sqrt()) / 2_f64);
        let expected_full_quarter_point = Tuple::new_point(0, 0, 1);

        assert_eq!(&half_quarter * &p, expected_half_quarter_point);
        assert_eq!(&full_quarter * &p, expected_full_quarter_point);

        let inv = inverse_4x4(&half_quarter)?;
        assert_eq!(&inv * &expected_half_quarter_point, p);
        Ok(())
    }

    #[test]
    fn rotation_y_tests() -> Result<()> {
        let p = Tuple::new_point(0, 0, 1);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        let expected_half_quarter_point =
            Tuple::new_point(((2_f64).sqrt()) / 2_f64, 0, ((2_f64).sqrt()) / 2_f64);
        let expected_full_quarter_point = Tuple::new_point(1, 0, 0);

        assert_eq!(&half_quarter * &p, expected_half_quarter_point);
        assert_eq!(&full_quarter * &p, expected_full_quarter_point);

        let inv = inverse_4x4(&half_quarter)?;
        assert_eq!(&inv * &expected_half_quarter_point, p);
        Ok(())
    }

    #[test]
    fn rotation_z_tests() -> Result<()> {
        let p = Tuple::new_point(0, 1, 0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        let expected_half_quarter_point =
            Tuple::new_point(-((2_f64).sqrt()) / 2_f64, ((2_f64).sqrt()) / 2_f64, 0);
        let expected_full_quarter_point = Tuple::new_point(-1, 0, 0);

        assert_eq!(&half_quarter * &p, expected_half_quarter_point);
        assert_eq!(&full_quarter * &p, expected_full_quarter_point);

        let inv = inverse_4x4(&half_quarter)?;
        assert_eq!(&inv * &expected_half_quarter_point, p);
        Ok(())
    }

    #[test]
    fn shearing_tests() -> Result<()> {
        let p = Tuple::new_point(2, 3, 4);

        // case 1
        let t1 = shearing(1, 0, 0, 0, 0, 0);
        let expected_shear_t1 = Tuple::new_point(5, 3, 4);
        assert_eq!(&t1 * &p, expected_shear_t1);

        // case 2
        let t2 = shearing(0, 1, 0, 0, 0, 0);
        let expected_shear_t2 = Tuple::new_point(6, 3, 4);
        assert_eq!(&t2 * &p, expected_shear_t2);

        // case 3
        let t3 = shearing(0, 0, 1, 0, 0, 0);
        let expected_shear_t3 = Tuple::new_point(2, 5, 4);
        assert_eq!(&t3 * &p, expected_shear_t3);

        // case 4
        let t4 = shearing(0, 0, 0, 1, 0, 0);
        let expected_shear_t4 = Tuple::new_point(2, 7, 4);
        assert_eq!(&t4 * &p, expected_shear_t4);

        // case 5
        let t5 = shearing(0, 0, 0, 0, 1, 0);
        let expected_shear_t5 = Tuple::new_point(2, 3, 6);
        assert_eq!(&t5 * &p, expected_shear_t5);

        // case 6
        let t6 = shearing(0, 0, 0, 0, 0, 1);
        let expected_shear_t6 = Tuple::new_point(2, 3, 7);
        assert_eq!(&t6 * &p, expected_shear_t6);

        Ok(())
    }

    #[test]
    fn chaining_transforms() -> Result<()> {
        let p = Tuple::new_point(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5, 5, 5);
        let c = translation(10, 5, 7);

        // Case 1: without chaining
        let result_rotate = &a * &p;
        assert_eq!(result_rotate, Tuple::new_point(1, -1, 0));

        let result_scale = &b * &result_rotate;
        assert_eq!(result_scale, Tuple::new_point(5, -5, 0));

        let result_translate = &c * &result_scale;
        assert_eq!(result_translate, Tuple::new_point(15, 0, 7));

        // Case 2: with chaining
        let chained_transform = a.multiply(&b)?.multiply(&c)?;
        assert_eq!(&chained_transform * &p, result_translate);

        Ok(())
    }
}
