use typed_floats::tf64::Positive;

use crate::{
    color::Color,
    patterns::{PatternType, Solid},
    utils::float_equals,
};

#[derive(Debug, Clone, Copy, PartialOrd)]
/// Data structure capturing attributes such as surface color,
/// shininess, diffusion, specular, and ambience. These materials
/// are then associated with objects to give them these properties.
pub struct Material {
    pattern: PatternType,
    ambient: Positive,
    diffuse: Positive,
    specular: Positive,
    shininess: Positive,
}

impl Material {
    /// Create a new instance of the default material
    pub fn new(
        pattern: PatternType,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Self {
        Self {
            pattern,
            ambient: Positive::new(ambient).unwrap(),
            diffuse: Positive::new(diffuse).unwrap(),
            specular: Positive::new(specular).unwrap(),
            shininess: Positive::new(shininess).unwrap(),
        }
    }

    pub fn get_pattern(&self) -> &PatternType {
        &self.pattern
    }

    pub fn set_pattern(&mut self, pattern: PatternType) {
        self.pattern = pattern
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
            pattern: PatternType::Solid(Solid::from(Color::white())),
            ambient: Positive::new(0.1).unwrap(),
            diffuse: Positive::new(0.9).unwrap(),
            specular: Positive::new(0.9).unwrap(),
            shininess: Positive::new(200.0).unwrap(),
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
            && float_equals(&(self.ambient.into()), &(other.ambient).into())
            && float_equals(&(self.diffuse.into()), &(other.diffuse).into())
            && float_equals(&(self.specular.into()), &(other.specular).into())
            && float_equals(&(self.shininess.into()), &(other.shininess).into())
    }
}

#[cfg(test)]
mod test {
    use super::Material;
    use crate::{
        color::Color,
        patterns::{PatternType, Solid},
    };

    #[test]
    fn create_default_material() {
        let m = Material::default();

        assert_eq!(
            m.get_pattern(),
            &PatternType::Solid(Solid::from(Color::new(1, 1, 1)))
        );
        assert_eq!(m.get_ambient(), 0.1);
        assert_eq!(m.get_diffuse(), 0.9);
        assert_eq!(m.get_specular(), 0.9);
        assert_eq!(m.get_shininess(), 200.0);
    }
}
