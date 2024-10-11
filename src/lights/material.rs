use typed_floats::tf64::Positive;

use crate::{color::Color, utils::float_equals};

#[derive(Debug, Clone, Copy)]
/// Data structure capturing attributes such as surface color,
/// shininess, diffusion, specular, and ambience. These materials
/// are then associated with objects to give them these properties.
pub struct Material {
    color: Color,
    ambient: Positive,
    diffuse: Positive,
    specular: Positive,
    shininess: Positive,
}

impl Material {
    /// Create a new instance of the default material
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the color of the material
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Set the color value for a material
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Get the ambient attribute for a material
    pub fn get_ambient(&self) -> f64 {
        self.ambient.into()
    }

    /// Set the ambient attribute for a material
    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = Positive::new(ambient).unwrap()
    }

    /// Get the diffuse attribute for a material
    pub fn get_diffuse(&self) -> f64 {
        self.diffuse.into()
    }

    /// Set the diffuse attribute for a material
    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = Positive::new(diffuse).unwrap();
    }

    /// Get the specular attribute for a material
    pub fn get_specular(&self) -> f64 {
        self.specular.into()
    }

    /// Set the specular attribute for a material
    pub fn set_specular(&mut self, specular: f64) {
        self.specular = Positive::new(specular).unwrap();
    }

    /// Get the shininess attribute for a material
    pub fn get_shininess(&self) -> f64 {
        self.shininess.into()
    }

    /// Set the shininess attribute for a material
    pub fn set_shininess(&mut self, shininess: f64) {
        self.shininess = Positive::new(shininess).unwrap();
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1, 1, 1),
            ambient: Positive::new(0.1).unwrap(),
            diffuse: Positive::new(0.9).unwrap(),
            specular: Positive::new(0.9).unwrap(),
            shininess: Positive::new(200.0).unwrap(),
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && float_equals(&(self.ambient.into()), &(other.ambient).into())
            && float_equals(&(self.diffuse.into()), &(other.diffuse).into())
            && float_equals(&(self.specular.into()), &(other.specular).into())
            && float_equals(&(self.shininess.into()), &(other.shininess).into())
    }
}

#[cfg(test)]
mod test {
    use super::Material;
    use crate::color::Color;

    #[test]
    fn create_default_material() {
        let m = Material::new();

        assert_eq!(m.get_color(), Color::new(1, 1, 1));
        assert_eq!(m.get_ambient(), 0.1);
        assert_eq!(m.get_diffuse(), 0.9);
        assert_eq!(m.get_specular(), 0.9);
        assert_eq!(m.get_shininess(), 200.0);
    }
}
