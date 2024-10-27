use crate::{
    canvas::Canvas,
    intersections::Ray,
    matrix::{inverse_4x4, Matrix},
    spatial::Tuple,
    world::World,
};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
/// Data structure that represents a camera that can
/// be used to render images of worlds
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix<4, 4>,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    /// Create a new camera of size with a specific configuration of it's
    /// size and field of view
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::<4, 4>::identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    /// Get the width of the camera
    pub fn get_hsize(&self) -> usize {
        self.hsize
    }

    /// Get the height of the camera
    pub fn get_vsize(&self) -> usize {
        self.vsize
    }

    /// Get the field of view of the camera
    pub fn get_field_of_view(&self) -> f64 {
        self.field_of_view
    }

    /// Get the transform matrix for the camera
    pub fn get_transform(&self) -> &Matrix<4, 4> {
        &self.transform
    }

    /// Mutate the camera by setting a new transform matrix
    pub fn set_transform(&mut self, transform: Matrix<4, 4>) {
        self.transform = transform;
    }

    /// Get the calculated pixel size for the camera based on the
    /// height, width, and field of view
    pub fn get_pixel_size(&self) -> f64 {
        self.pixel_size
    }

    /// Calculates a ray that pass through the given pixel coordinate (px,py)
    /// on the camera canvas
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Result<Ray> {
        // The offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let pixel = &inverse_4x4(&self.transform)? * &Tuple::point(world_x, world_y, -1);
        let origin = &inverse_4x4(&self.transform)? * &Tuple::point(0, 0, 0);
        let direction = (&pixel - &origin).normalize();

        Ray::new(origin, direction)
    }

    /// Uses the camera to render an image of the given world
    pub fn render(&self, world: &World) -> Result<Canvas> {
        let mut image = Canvas::new(self.get_hsize(), self.get_vsize());

        for y in 0..(self.vsize - 1) {
            for x in 0..(self.hsize - 1) {
                let ray = self.ray_for_pixel(x, y)?;
                let color = world.color_at(&ray)?;
                image.write_pixel(x, y, color)?;
            }
        }

        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use super::Camera;
    use crate::{
        color::Color,
        matrix::{rotation_y, translation, view_transform},
        spatial::Tuple,
        utils::float_equals,
        world::World,
    };
    use anyhow::Result;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn constructing_new_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.get_hsize(), hsize);
        assert_eq!(c.get_vsize(), vsize);
        assert_eq!(c.get_field_of_view(), field_of_view);
    }

    #[test]
    fn calculate_pixel_size() {
        let c1 = Camera::new(200, 125, PI / 2.0);
        assert!(float_equals(&c1.get_pixel_size(), &0.01));

        let c2 = Camera::new(125, 200, PI / 2.0);
        assert!(float_equals(&c2.get_pixel_size(), &0.01));
    }

    #[test]
    fn ray_for_pixel_works() -> Result<()> {
        let mut c = Camera::new(201, 101, PI / 2.0);

        // Constructing a ray through the center of the canvas
        let r = c.ray_for_pixel(100, 50)?;
        assert_eq!(r.origin, Tuple::point(0, 0, 0));
        assert_eq!(r.direction, Tuple::vector(0, 0, -1));

        // Constructing a ray through the corner of the canvas
        let r = c.ray_for_pixel(0, 0)?;
        assert_eq!(r.origin, Tuple::point(0, 0, 0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));

        // Constructing a ray when the camera is transformed
        c.set_transform((&rotation_y(PI / 4.0) * &translation(0, -2, 5))?);
        let r = c.ray_for_pixel(100, 50)?;
        assert_eq!(r.origin, Tuple::point(0, 2, -5));
        assert_eq!(r.direction, Tuple::vector(SQRT_2 / 2.0, 0, -SQRT_2 / 2.0));

        Ok(())
    }

    #[test]
    fn rendering_a_world_with_a_camera() -> Result<()> {
        let w = World::default();

        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::point(0, 0, -5);
        let to = Tuple::point(0, 0, 0);
        let up = Tuple::vector(0, 1, 0);
        c.set_transform(view_transform(&from, &to, &up));

        let image = c.render(&w)?;
        assert_eq!(image.pixel_at(5, 5)?, &Color::new(0.38066, 0.47583, 0.2855));

        Ok(())
    }
}
