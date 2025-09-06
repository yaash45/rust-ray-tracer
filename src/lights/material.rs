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
    pub pattern: PatternType,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Solid::from(Color::white()).into(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
            && float_equals(&self.ambient, &other.ambient)
            && float_equals(&self.diffuse, &other.diffuse)
            && float_equals(&self.specular, &other.specular)
            && float_equals(&self.shininess, &other.shininess)
            && float_equals(&self.reflective, &other.reflective)
    }
}

#[cfg(test)]
mod test {
    use super::Material;
    use crate::{color::Color, patterns::Solid};

    #[test]
    fn create_default_material() {
        let m = Material::default();

        assert_eq!(m.pattern, Solid::from(Color::new(1, 1, 1)).into());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
        assert_eq!(m.reflective, 0.0);
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
