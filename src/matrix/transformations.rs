use crate::{matrix::Matrix, spatial::Tuple};

/// Trait used to abstract away the process of getting and setting the transformation matrix
/// for various objects in the raytracer. It is used to implement the `get_transform` and
/// `set_transform` methods for Shapes and Patterns.
pub trait Transformable {
    /// Gets the transformation matrix for this object
    fn get_transform(&self) -> &Matrix<4, 4>;

    /// Sets the transformation matrix for this object
    fn set_transform(&mut self, transform_matrix: Matrix<4, 4>);
}

/// Gets a 4x4 transformation matrix that can be used to translate tuples in 3D space
///
/// ```
/// use raytracer::{matrix::translation, spatial::Tuple};
///
/// let translator = translation(5, -3, 2);
/// let point = Tuple::point(1, 1, 1);
/// let expected_translated_point = Tuple::point(6, -2, 3);
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
/// let point = Tuple::point(1, 1, 1);
/// let expected_scaled_point = Tuple::point(2, 2, 2);
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

/// Gets a view transform to for the eye vector based on the provided
/// from, to, and up Tuples for the world
pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix<4, 4> {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    let orientation = Matrix::from([
        [left.get_x(), left.get_y(), left.get_z(), 0.0],
        [true_up.get_x(), true_up.get_y(), true_up.get_z(), 0.0],
        [-forward.get_x(), -forward.get_y(), -forward.get_z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let translation_transform = translation(-from.get_x(), -from.get_y(), -from.get_z());

    let view_transform_result = &orientation * &translation_transform;

    // Just returning a identity matrix if multiplication went wrong (it _probably_ won't)
    match view_transform_result {
        Ok(vt) => vt,
        Err(_) => Matrix::<4, 4>::identity(),
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::{rotation_x, rotation_y, rotation_z, scaling, shearing, translation};
    use crate::matrix::transformations::view_transform;
    use crate::matrix::{inverse_4x4, Matrix};
    use crate::spatial::Tuple;
    use anyhow::Result;

    #[test]
    fn translation_of_tuples() -> Result<()> {
        let point = Tuple::point(-3, 4, 5);
        let transform = translation(5, -3, 2);
        let expected_destination = Tuple::point(2, 1, 7);

        // Translating the input point to the expected destination
        assert_eq!(&transform * &point, expected_destination);

        // When using the inverse of the translation matrix, we can
        // go back from the destination to the original point location
        let inv = inverse_4x4(&transform)?;
        assert_eq!(&inv * &expected_destination, point);

        // The translation of a vector is the same vector
        let vector = Tuple::vector(-3, 4, 5);
        assert_eq!(&transform * &vector, vector);

        Ok(())
    }

    #[test]
    fn scaling_of_tuples() -> Result<()> {
        let transform = scaling(2, 3, 4);
        let point = Tuple::point(-4, 6, 8);
        let vector = Tuple::vector(-4, 6, 8);

        // Scaling applied to the point
        let expected_scaled_point = Tuple::point(-8, 18, 32);
        assert_eq!(&transform * &point, expected_scaled_point);

        // Scaling applied to a vector
        let expected_scaled_vector = Tuple::vector(-8, 18, 32);
        assert_eq!(&transform * &vector, expected_scaled_vector);

        // Scaling by the inverse will shrink the tuple instead of growing it
        let inv = inverse_4x4(&transform)?;
        assert_eq!(&inv * &expected_scaled_point, point);
        assert_eq!(&inv * &expected_scaled_vector, vector);

        // We can use scaling to reflect a point along any axes
        let reflect_x = scaling(-1, 1, 1);
        let reflected_point = Tuple::point(-point.get_x(), point.get_y(), point.get_z());
        assert_eq!(&reflect_x * &point, reflected_point);

        Ok(())
    }

    #[test]
    fn rotation_x_tests() -> Result<()> {
        let p = Tuple::point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        let expected_half_quarter_point =
            Tuple::point(0, ((2_f64).sqrt()) / 2_f64, ((2_f64).sqrt()) / 2_f64);
        let expected_full_quarter_point = Tuple::point(0, 0, 1);

        assert_eq!(&half_quarter * &p, expected_half_quarter_point);
        assert_eq!(&full_quarter * &p, expected_full_quarter_point);

        let inv = inverse_4x4(&half_quarter)?;
        assert_eq!(&inv * &expected_half_quarter_point, p);
        Ok(())
    }

    #[test]
    fn rotation_y_tests() -> Result<()> {
        let p = Tuple::point(0, 0, 1);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        let expected_half_quarter_point =
            Tuple::point(((2_f64).sqrt()) / 2_f64, 0, ((2_f64).sqrt()) / 2_f64);
        let expected_full_quarter_point = Tuple::point(1, 0, 0);

        assert_eq!(&half_quarter * &p, expected_half_quarter_point);
        assert_eq!(&full_quarter * &p, expected_full_quarter_point);

        let inv = inverse_4x4(&half_quarter)?;
        assert_eq!(&inv * &expected_half_quarter_point, p);
        Ok(())
    }

    #[test]
    fn rotation_z_tests() -> Result<()> {
        let p = Tuple::point(0, 1, 0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        let expected_half_quarter_point =
            Tuple::point(-((2_f64).sqrt()) / 2_f64, ((2_f64).sqrt()) / 2_f64, 0);
        let expected_full_quarter_point = Tuple::point(-1, 0, 0);

        assert_eq!(&half_quarter * &p, expected_half_quarter_point);
        assert_eq!(&full_quarter * &p, expected_full_quarter_point);

        let inv = inverse_4x4(&half_quarter)?;
        assert_eq!(&inv * &expected_half_quarter_point, p);
        Ok(())
    }

    #[test]
    fn shearing_tests() -> Result<()> {
        let p = Tuple::point(2, 3, 4);

        // case 1
        let t1 = shearing(1, 0, 0, 0, 0, 0);
        let expected_shear_t1 = Tuple::point(5, 3, 4);
        assert_eq!(&t1 * &p, expected_shear_t1);

        // case 2
        let t2 = shearing(0, 1, 0, 0, 0, 0);
        let expected_shear_t2 = Tuple::point(6, 3, 4);
        assert_eq!(&t2 * &p, expected_shear_t2);

        // case 3
        let t3 = shearing(0, 0, 1, 0, 0, 0);
        let expected_shear_t3 = Tuple::point(2, 5, 4);
        assert_eq!(&t3 * &p, expected_shear_t3);

        // case 4
        let t4 = shearing(0, 0, 0, 1, 0, 0);
        let expected_shear_t4 = Tuple::point(2, 7, 4);
        assert_eq!(&t4 * &p, expected_shear_t4);

        // case 5
        let t5 = shearing(0, 0, 0, 0, 1, 0);
        let expected_shear_t5 = Tuple::point(2, 3, 6);
        assert_eq!(&t5 * &p, expected_shear_t5);

        // case 6
        let t6 = shearing(0, 0, 0, 0, 0, 1);
        let expected_shear_t6 = Tuple::point(2, 3, 7);
        assert_eq!(&t6 * &p, expected_shear_t6);

        Ok(())
    }

    #[test]
    fn chaining_transforms() -> Result<()> {
        let p = Tuple::point(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5, 5, 5);
        let c = translation(10, 5, 7);

        // Case 1: without chaining
        let result_rotate = &a * &p;
        assert_eq!(result_rotate, Tuple::point(1, -1, 0));

        let result_scale = &b * &result_rotate;
        assert_eq!(result_scale, Tuple::point(5, -5, 0));

        let result_translate = &c * &result_scale;
        assert_eq!(result_translate, Tuple::point(15, 0, 7));

        // Case 2: with chaining
        let chained_transform = a.multiply(&b)?.multiply(&c)?;
        assert_eq!(&chained_transform * &p, result_translate);

        Ok(())
    }

    #[test]
    fn default_orientation_view_transform() {
        let from = Tuple::point(0, 0, 0);
        let to = Tuple::point(0, 0, -1);
        let up = Tuple::vector(0, 1, 0);

        assert_eq!(view_transform(&from, &to, &up), Matrix::<4, 4>::identity());
    }

    #[test]
    fn view_transformation_looking_in_positive_z_direction() {
        let from = Tuple::point(0, 0, 0);
        let to = Tuple::point(0, 0, 1);
        let up = Tuple::vector(0, 1, 0);

        assert_eq!(view_transform(&from, &to, &up), scaling(-1, 1, -1));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Tuple::point(0, 0, 8);
        let to = Tuple::point(0, 0, 0);
        let up = Tuple::vector(0, 1, 0);

        assert_eq!(view_transform(&from, &to, &up), translation(0, 0, -8));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Tuple::point(1, 3, 2);
        let to = Tuple::point(4, -2, 8);
        let up = Tuple::vector(1, 1, 0);

        let expected = Matrix::from([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.767715, 0.60609, 0.121218, -2.82843],
            [-0.35856, 0.59761, -0.71714, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let actual = view_transform(&from, &to, &up);

        assert_eq!(actual, expected);
    }
}
