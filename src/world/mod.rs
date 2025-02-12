use crate::{
    color::Color,
    intersections::{hit, Computations, Intersection, Ray},
    lights::{lighting, PointLight},
    matrix::scaling,
    patterns::{PatternType, Solid},
    shapes::{Intersect, Shape, Sphere},
    spatial::Tuple,
};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
/// Data structure representing the world that contains
/// objects and a light source
pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Shape>,
}

impl World {
    /// Creates a new default world
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty world
    pub fn empty() -> Self {
        Self {
            light: None,
            objects: vec![],
        }
    }

    /// Return a reference to the light in the world
    pub fn get_light(&self) -> Option<&PointLight> {
        self.light.as_ref()
    }

    /// Set the world light source
    pub fn set_light(&mut self, light: Option<PointLight>) {
        self.light = light;
    }

    /// Add an object to the world
    pub fn add_object(&mut self, obj: Shape) {
        self.objects.push(obj);
    }

    /// Get a count of number of objects in the world
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Determines if a point in the world is shadowed or not
    pub fn is_shadowed(&self, point: &Tuple) -> Result<bool> {
        if self.light.is_none() {
            return Ok(false);
        }

        let v = &self.light.unwrap().position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*point, direction)?;
        let h = hit(self.intersect_world(&r)?);

        if let Some(h) = h {
            Ok(h.t < distance)
        } else {
            Ok(false)
        }
    }

    /// Finds and returns all the intersections of the given ray
    /// with the world
    fn intersect_world(&self, ray: &Ray) -> Result<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = vec![];
        for o in &self.objects {
            let mut intersections = o.intersect(ray)?;
            xs.append(&mut intersections);
        }

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Ok(xs)
    }

    /// Given a set of pre-computed state values of the world,
    /// calculate the color of a hit in the world
    fn shade_hit(&self, comps: &Computations) -> Result<Color> {
        if self.light.is_none() {
            return Ok(Color::black());
        }

        lighting(
            &comps.get_object().get_material(),
            comps.get_object(),
            self.light.as_ref().unwrap(),
            comps.get_point(),
            comps.get_eyev(),
            comps.get_normalv(),
            self.is_shadowed(comps.get_over_point())?, // placeholder until shadows are accounted for
        )
    }

    /// This method calculates all the intersections of a given ray
    /// in the world with the objects in it, and uses this information
    /// to find the color at the hits from the input ray.
    pub fn color_at(&self, ray: &Ray) -> Result<Color> {
        let xs = self.intersect_world(ray)?;
        let h = hit(xs);

        if h.is_none() {
            return Ok(Color::black());
        }

        let comps = Computations::prepare_computations(h.as_ref().unwrap(), ray)?;
        self.shade_hit(&comps)
    }
}

impl Default for World {
    fn default() -> Self {
        let light_source =
            PointLight::new(Tuple::point(-10, 10, -10), Color::new(1, 1, 1)).unwrap();

        let mut s1 = Sphere::default();
        s1.material
            .set_pattern(PatternType::Solid(Solid::new(Color::new(0.8, 1.0, 0.6))));
        s1.material.set_diffuse(0.7);
        s1.material.set_specular(0.2);

        let mut s2 = Sphere::default();
        s2.transform_matrix = scaling(0.5, 0.5, 0.5);

        Self {
            light: Some(light_source),
            objects: vec![Shape::Sphere(s1), Shape::Sphere(s2)],
        }
    }
}

#[cfg(test)]
mod test {
    use super::World;
    use crate::{
        color::Color,
        intersections::{Computations, Intersection, Ray},
        lights::PointLight,
        matrix::translation,
        patterns::Pattern,
        shapes::{Shape, Sphere},
        spatial::Tuple,
    };
    use anyhow::Result;

    #[test]
    fn new_world_is_empty() {
        let w = World::empty();
        assert_eq!(w.get_light(), None);
        assert_eq!(w.object_count(), 0);
    }

    #[test]
    fn default_world_is_built_correctly() {
        let w = World::default();

        assert!(w.get_light().is_some());
        assert_eq!(w.object_count(), 2);
    }

    #[test]
    fn intersect_world_default() -> Result<()> {
        let w = World::default();
        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;

        let xs = w.intersect_world(&ray)?;

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);

        Ok(())
    }

    #[test]
    fn shading_an_intersection() -> Result<()> {
        let w = World::default();
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;

        // Ensure that we have two objects in our world
        assert_eq!(w.object_count(), 2);
        let i = Intersection::new(4, w.objects[0]);
        let comps = Computations::prepare_computations(&i, &r)?;

        let c = w.shade_hit(&comps)?;

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));

        Ok(())
    }

    #[test]
    fn shading_an_intersection_from_the_inside() -> Result<()> {
        let mut w = World::default();
        w.set_light(Some(PointLight::new(
            Tuple::point(0, 0.25, 0),
            Color::new(1, 1, 1),
        )?));

        let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1))?;

        // Ensure that we have two objects in our world
        assert_eq!(w.object_count(), 2);
        let i = Intersection::new(0.5, w.objects[1]);
        let comps = Computations::prepare_computations(&i, &r)?;

        let c = w.shade_hit(&comps)?;

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));

        Ok(())
    }

    #[test]
    fn color_at_when_ray_misses() -> Result<()> {
        let w = World::default();
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 1, 0))?;
        let c = w.color_at(&r)?;
        assert_eq!(c, Color::black());
        Ok(())
    }

    #[test]
    fn color_at_when_a_ray_hits() -> Result<()> {
        let w = World::default();
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let c = w.color_at(&r)?;
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
        Ok(())
    }

    #[test]
    fn color_at_when_intersection_is_behind_ray() -> Result<()> {
        let mut w = World::default();
        let mut m = w.objects[0].get_material();
        m.set_ambient(1.0);
        w.objects[0].set_material(m);

        let mut m = w.objects[1].get_material();
        m.set_ambient(1.0);
        w.objects[1].set_material(m);

        let r = Ray::new(Tuple::point(0, 0, 0.75), Tuple::vector(0, 0, -1))?;
        let c = w.color_at(&r)?;
        assert_eq!(
            c,
            w.objects[1]
                .get_material()
                .get_pattern()
                .pattern_at(&Tuple::point(0, 0, -1))
        );
        Ok(())
    }

    #[test]
    fn is_shadowed_works() -> Result<()> {
        let w = World::default();

        // There is no shadow when nothing is collinear with point and light
        let p1 = Tuple::point(0, 10, 0);
        assert!(!w.is_shadowed(&p1)?);

        // The shadow when an object is between the point and the light
        let p2 = Tuple::point(10, -10, 10);
        assert!(w.is_shadowed(&p2)?);

        // There is no shadow when an object is behind the light
        let p3 = Tuple::point(-20, 20, -20);
        assert!(!w.is_shadowed(&p3)?);

        // There is no shadow when an object is behind the point
        let p4 = Tuple::point(-2, 2, -2);
        assert!(!w.is_shadowed(&p4)?);

        Ok(())
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() -> Result<()> {
        let light = PointLight::new(Tuple::point(0, 0, -10), Color::new(1, 1, 1))?;
        let mut w = World::empty();
        w.set_light(Some(light));

        let s1 = Sphere::default();
        w.add_object(Shape::Sphere(s1));

        let mut s2 = Sphere::default();
        s2.transform_matrix = translation(0, 0, 10);
        w.add_object(Shape::Sphere(s2));

        let r = Ray::new(Tuple::point(0, 0, 5), Tuple::vector(0, 0, 1))?;
        let i = Intersection::new(4, Shape::Sphere(s2));

        let comps = Computations::prepare_computations(&i, &r)?;
        let c = w.shade_hit(&comps)?;

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));

        Ok(())
    }
}
