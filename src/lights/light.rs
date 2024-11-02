use crate::{color::Color, intersections::reflect, spatial::Tuple};
use anyhow::{Error, Result};

use super::Material;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Data structure representing a light source. A light source
/// has a position in space, and a specific color
pub struct PointLight {
    intensity: Color,
    pub(crate) position: Tuple,
}

impl PointLight {
    /// Create a new light source at `position` of color `intensity`
    ///
    /// Note: This returns a result because it validates the type
    /// of the input tuple to ensure we are passing in a point and
    /// not a vector.
    pub fn new(position: Tuple, intensity: Color) -> Result<Self> {
        if position.is_a_vector() {
            Err(Error::msg("position must be a Point not a Vector"))
        } else {
            Ok(Self {
                intensity,
                position,
            })
        }
    }
}

/// Calculates the color value for a light source hitting a material
/// by simulating the reflection of light off the given material.
///
/// The algorithm supporting this function is based on the
/// Phong reflection model
pub fn lighting(
    material: &Material,
    point_light: &PointLight,
    position: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool,
) -> Color {
    // combine surface color with the light's intensity/color
    let effective_color = material.get_color() * point_light.intensity;

    // find the direction to the light source
    let lightv = (&point_light.position - position).normalize();

    // compute ambient contribution
    let ambient = effective_color * material.get_ambient();

    // light_dot_normal represents the cosine of the angle between the​
    // light vector and the normal vector. A negative number means the​
    // light is on the other side of the surface.
    let light_dot_normal = lightv.dot(normalv);
    let mut diffuse = Color::black();
    let mut specular = Color::black();

    if light_dot_normal >= 0.0 {
        // compute the diffuse contribution
        diffuse = effective_color * material.get_diffuse() * light_dot_normal;

        // reflect_dot_eye represents the cosine angle between the
        // reflection vector and the eye vector. Negative number
        // means the light reflects away from the eye
        let reflectv = reflect(&(&lightv * -1.0), normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye >= 0.0 {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(material.get_shininess());
            specular = point_light.intensity * material.get_specular() * factor;
        }
    }

    if in_shadow {
        return ambient;
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::{lighting, Material, PointLight};
    use crate::{color::Color, spatial::Tuple};
    use anyhow::Result;

    #[test]
    fn lighting_with_eye_between_light_and_surface() -> Result<()> {
        let m = Material::default();
        let position = Tuple::point(0, 0, 0);

        let eyev = Tuple::vector(0, 0, -1);
        let normal = Tuple::vector(0, 0, -1);
        let point_light = PointLight::new(Tuple::point(0, 0, -10), Color::new(1, 1, 1))?;
        let in_shadow = false;

        let result = lighting(&m, &point_light, &position, &eyev, &normal, in_shadow);
        let expected = Color::new(1.9, 1.9, 1.9);

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn light_between_source_and_surface_offset_45_degrees() -> Result<()> {
        let m = Material::default();
        let position = Tuple::point(0, 0, 0);

        let eyev = Tuple::vector(0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normal = Tuple::vector(0, 0, -1);
        let point_light = PointLight::new(Tuple::point(0, 0, -10), Color::new(1, 1, 1))?;
        let in_shadow = false;

        let result = lighting(&m, &point_light, &position, &eyev, &normal, in_shadow);
        let expected = Color::new(1, 1, 1);

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn eye_opposite_surface_light_source_offset_45_degrees() -> Result<()> {
        let m = Material::default();
        let position = Tuple::point(0, 0, 0);

        let eyev = Tuple::vector(0, 0, -1);
        let normal = Tuple::vector(0, 0, -1);
        let point_light = PointLight::new(Tuple::point(0, 10, -10), Color::new(1, 1, 1))?;
        let in_shadow = false;

        let result = lighting(&m, &point_light, &position, &eyev, &normal, in_shadow);
        let expected = Color::new(0.7364, 0.7364, 0.7364);

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn light_with_eye_in_path_of_reflection_vector() -> Result<()> {
        let m = Material::default();
        let position = Tuple::point(0, 0, 0);

        let eyev = Tuple::vector(0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normal = Tuple::vector(0, 0, -1);
        let point_light = PointLight::new(Tuple::point(0, 10, -10), Color::new(1, 1, 1))?;
        let in_shadow = false;

        let result = lighting(&m, &point_light, &position, &eyev, &normal, in_shadow);
        let expected = Color::new(1.6364, 1.6364, 1.6364);

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn light_behind_surface() -> Result<()> {
        let m = Material::default();
        let position = Tuple::point(0, 0, 0);

        let eyev = Tuple::vector(0, 0, -1);
        let normal = Tuple::vector(0, 0, -1);
        let point_light = PointLight::new(Tuple::point(0, 0, 10), Color::new(1, 1, 1))?;
        let in_shadow = false;

        let result = lighting(&m, &point_light, &position, &eyev, &normal, in_shadow);
        let expected = Color::new(0.1, 0.1, 0.1);

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn lighting_with_surface_in_shadow() -> Result<()> {
        let m = Material::default();
        let position = Tuple::point(0, 0, 0);

        let eyev = Tuple::vector(0, 0, -1);
        let normal = Tuple::vector(0, 0, -1);
        let point_light = PointLight::new(Tuple::point(0, 0, -10), Color::new(1, 1, 1))?;
        let in_shadow = true;

        let result = lighting(&m, &point_light, &position, &eyev, &normal, in_shadow);
        let expected = Color::new(0.1, 0.1, 0.1);

        assert_eq!(result, expected);

        Ok(())
    }
}
