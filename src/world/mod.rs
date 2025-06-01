use crate::{
    color::Color,
    intersections::{hit, Computations, Intersection, Ray},
    lights::{lighting, PointLight},
    matrix::scaling,
    patterns::Solid,
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
        let xs = self.intersect_world(&r)?;
        let h = hit(&xs);

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

    /// Compute the color of the intersection point based on the world
    /// given the current computation state.
    #[allow(unused)]
    fn shade_hit(&self, comps: &Computations) -> Result<Color> {
        self.shade_hit_helper(comps, 5)
    }

    /// Compute the color of the intersection point based on the world
    /// given the current computation state, and recursively handle
    /// reflection.
    fn shade_hit_helper(&self, comps: &Computations, remaining_iterations: usize) -> Result<Color> {
        if self.light.is_none() {
            return Ok(Color::black());
        }

        let surface = lighting(
            &comps.object.get_material(),
            &comps.object,
            self.light.as_ref().unwrap(),
            &comps.point,
            &comps.eyev,
            &comps.normalv,
            self.is_shadowed(&comps.over_point)?,
        )?;

        let reflected = self.reflected_color_helper(comps, remaining_iterations)?;
        let refracted = self.refracted_color(comps, remaining_iterations)?;

        Ok(surface + reflected + refracted)
    }

    /// Computes the reflected color at the intersection point, taking into account
    /// the material's reflective properties and a specified recursion depth.
    /// If the remaining iterations are zero or the material is not reflective, it
    /// returns black. Otherwise, it casts a reflection ray and computes the color
    /// recursively, attenuated by the material's reflectiveness.
    fn reflected_color_helper(
        &self,
        comps: &Computations,
        remaining_iterations: usize,
    ) -> Result<Color> {
        if remaining_iterations == 0 || comps.object.get_material().reflective == 0.0 {
            return Ok(Color::black());
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv)?;
        let color = self.color_at_helper(&reflect_ray, remaining_iterations - 1)?;

        Ok(color * comps.object.get_material().reflective)
    }

    /// Compute the reflected color at the intersection, given the current
    /// computation state. This uses a default recursion depth to determine
    /// the contribution of reflections to the final color.
    #[allow(unused)]
    fn reflected_color(&self, comps: &Computations) -> Result<Color> {
        self.reflected_color_helper(comps, 5)
    }

    /// Computes the refracted color at the intersection point, taking into account
    /// the material's transparency and a specified recursion depth.
    /// If the remaining iterations are zero or the material is not transparent, it
    /// returns black. Otherwise, it casts a refraction ray and computes the color
    /// recursively, attenuated by the material's transparency.
    pub fn refracted_color(
        &self,
        comps: &Computations,
        remaining_iterations: usize,
    ) -> Result<Color> {
        if remaining_iterations == 0 || comps.object.get_material().transparency == 0.0 {
            return Ok(Color::black());
        }
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(&comps.normalv);
        let sin2_t = n_ratio * n_ratio * (1.0 - (cos_i * cos_i));

        // Check if the condition for total internal reflection has
        // been met. If so, return black.
        if sin2_t > 1.0 {
            return Ok(Color::black());
        }

        let cos_t = f64::sqrt(1.0 - sin2_t);
        let direction = &comps.normalv * (n_ratio * cos_i - cos_t) - (&comps.eyev * n_ratio);
        let refracted_ray = Ray::new(comps.under_point, direction)?;

        Ok(
            self.color_at_helper(&refracted_ray, remaining_iterations - 1)?
                * comps.object.get_material().transparency,
        )
    }

    /// Calculates the color of the world at a given ray, recursively
    /// taking into account object materials and reflections up to a
    /// specified recursion depth. If the ray does not intersect with
    /// any objects, it returns black.
    fn color_at_helper(&self, ray: &Ray, remaining_iterations: usize) -> Result<Color> {
        let xs = self.intersect_world(ray)?;
        let h = hit(&xs);

        if h.is_none() {
            return Ok(Color::black());
        }

        let comps = Computations::prepare(h.unwrap(), ray, &xs)?;
        self.shade_hit_helper(&comps, remaining_iterations)
    }

    /// This method calculates all the intersections of a given ray
    /// in the world with the objects in it, and uses this information
    /// to find the color at the hits from the input ray.
    pub fn color_at(&self, ray: &Ray) -> Result<Color> {
        self.color_at_helper(ray, 5)
    }
}

impl Default for World {
    fn default() -> Self {
        let light_source =
            PointLight::new(Tuple::point(-10, 10, -10), Color::new(1, 1, 1)).unwrap();

        let mut s1 = Sphere::default();
        s1.material.pattern = Solid::new(Color::new(0.8, 1.0, 0.6)).into();
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

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
    use std::f64::consts::SQRT_2;

    use super::World;
    use crate::{
        color::Color,
        intersections::{Computations, Intersection, Ray},
        lights::{Material, PointLight},
        matrix::{translation, Transformable},
        patterns::{Pattern, Solid, TestPattern},
        shapes::{Plane, Shape, Sphere},
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
        let comps = Computations::prepare(&i, &r, &[])?;

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
        let comps = Computations::prepare(&i, &r, &[])?;

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
        m.ambient = 1.0;
        w.objects[0].set_material(m);

        let mut m = w.objects[1].get_material();
        m.ambient = 1.0;
        w.objects[1].set_material(m);

        let r = Ray::new(Tuple::point(0, 0, 0.75), Tuple::vector(0, 0, -1))?;
        let c = w.color_at(&r)?;
        assert_eq!(
            c,
            w.objects[1]
                .get_material()
                .pattern
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

        let comps = Computations::prepare(&i, &r, &[])?;
        let c = w.shade_hit(&comps)?;

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));

        Ok(())
    }
    #[test]
    fn reflected_color_for_non_reflective_material() -> Result<()> {
        // Arrange
        let w = World::default();
        let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1))?;
        let shape = w.objects[1];
        shape.get_material().ambient = 1.0;
        let i = Intersection::new(1.0, shape);

        // Act
        let comps = Computations::prepare(&i, &r, &[])?;
        let color = w.reflected_color(&comps)?;

        // Assert
        assert_eq!(color, Color::black());

        Ok(())
    }

    #[test]
    fn reflected_color_for_reflective_material() -> Result<()> {
        // Arrange
        let mut w = World::default();
        let mut shape = Shape::Plane(Plane::default());
        let mut material = shape.get_material();
        material.reflective = 0.5;
        shape.set_material(material);

        shape.set_transform(translation(0, -1, 0));
        w.add_object(shape);

        let r = Ray::new(
            Tuple::point(0, 0, -3),
            Tuple::vector(0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        )?;
        let i = Intersection::new(SQRT_2, shape);

        // Act
        let comps = Computations::prepare(&i, &r, &[])?;
        let color = w.reflected_color(&comps)?;

        // Assert
        assert_eq!(color, Color::new(0.190332, 0.237915, 0.142749));

        Ok(())
    }

    #[test]
    fn shade_hit_with_reflective_material() -> Result<()> {
        // Arrange
        let mut w = World::default();
        let mut shape = Shape::Plane(Plane::default());
        let mut material = shape.get_material();
        material.reflective = 0.5;
        shape.set_material(material);

        assert_eq!(shape.get_material().reflective, 0.5);
        shape.set_transform(translation(0, -1, 0));
        w.add_object(shape);

        let r = Ray::new(
            Tuple::point(0, 0, -3),
            Tuple::vector(0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        )?;
        let i = Intersection::new(SQRT_2, shape);

        // Act
        let comps = Computations::prepare(&i, &r, &[])?;
        let color = w.shade_hit(&comps)?;

        let expected_color = Color::new(0.87677, 0.92436, 0.82918);

        // Assert
        assert_eq!(color, expected_color);

        Ok(())
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World {
            light: Some(PointLight::new(Tuple::point(0, 0, 0), Color::new(1, 1, 1)).unwrap()),
            objects: vec![],
        };

        let mut lower = Shape::Plane(Plane::default());
        let material = Material {
            reflective: 1.0,
            ..Default::default()
        };
        lower.set_material(material);
        lower.set_transform(translation(0, -1, 0));
        w.add_object(lower);

        let mut upper = Shape::Plane(Plane::default());
        upper.set_material(material);
        upper.set_transform(translation(0, 1, 0));
        w.add_object(upper);

        let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 1, 0)).unwrap();

        // If this doesn't cause a SIGABRT, the test passes
        let _ = w.color_at(&r);
    }

    #[test]
    fn refracted_color_with_opaque_surface() -> Result<()> {
        let w = World::default();
        let shape = w.objects[0];
        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let xs = [Intersection::new(4, shape), Intersection::new(6, shape)];
        let comps = Computations::prepare(&xs[0], &ray, &xs)?;

        assert_eq!(w.refracted_color(&comps, 5)?, Color::black());

        Ok(())
    }

    #[test]
    fn refracted_color_at_maximum_recursive_depth() -> Result<()> {
        let w = World::default();
        let mut shape = w.objects[0];
        let mut mat = shape.get_material();

        mat.transparency = 1.0;
        mat.refractive_index = 1.5;
        shape.set_material(mat);

        let ray = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1))?;
        let xs = [Intersection::new(4, shape), Intersection::new(6, shape)];
        let comps = Computations::prepare(&xs[0], &ray, &xs)?;

        assert_eq!(w.refracted_color(&comps, 0)?, Color::black());

        Ok(())
    }

    #[test]
    fn refracted_color_with_total_internal_reflection() -> Result<()> {
        let w = World::default();
        let mut shape = w.objects[0];
        let mut mat = shape.get_material();

        mat.transparency = 1.0;
        mat.refractive_index = 1.5;
        shape.set_material(mat);

        let ray = Ray::new(Tuple::point(0, 0, SQRT_2 / 2.0), Tuple::vector(0, 1, 0))?;
        let xs = [
            Intersection::new(-SQRT_2 / 2.0, shape),
            Intersection::new(SQRT_2 / 2.0, shape),
        ];
        let comps = Computations::prepare(&xs[1], &ray, &xs)?;

        assert_eq!(w.refracted_color(&comps, 5)?, Color::black());

        Ok(())
    }

    #[test]
    fn refracted_color_with_a_refracted_ray() -> Result<()> {
        let mut w = World::default();

        let mut a = w.objects[0];
        let mut mat_a = a.get_material();
        mat_a.ambient = 1.0;
        mat_a.pattern = TestPattern::new().into();
        a.set_material(mat_a);
        w.objects[0].set_material(mat_a);

        let b = w.objects[1];
        let mut mat_b = b.get_material();
        mat_b.transparency = 1.0;
        mat_b.refractive_index = 1.5;
        w.objects[1].set_material(mat_b);

        let ray = Ray::new(Tuple::point(0, 0, 0.1), Tuple::vector(0, 1, 0))?;
        let xs = [
            Intersection::new(-0.9899, w.objects[0]),
            Intersection::new(-0.4899, w.objects[1]),
            Intersection::new(0.4899, w.objects[1]),
            Intersection::new(0.9899, w.objects[0]),
        ];

        let comps = Computations::prepare(&xs[2], &ray, &xs)?;

        assert_eq!(
            w.refracted_color(&comps, 5)?,
            Color::new(0, 0.99888, 0.04725)
        );

        Ok(())
    }

    #[test]
    fn shade_hit_with_transparent_material() -> Result<()> {
        let mut w = World::default();

        let floor = Shape::Plane(Plane::new(
            translation(0, -1, 0),
            Material {
                transparency: 0.5,
                refractive_index: 1.5,
                ..Default::default()
            },
        ));

        w.add_object(floor);

        let ball = Shape::Sphere(Sphere::new(
            translation(0, -3.5, -0.5),
            Material {
                ambient: 0.5,
                pattern: Solid::new(Color::red()).into(),
                ..Default::default()
            },
        ));

        w.add_object(ball);

        let r = Ray::new(
            Tuple::point(0, 0, -3),
            Tuple::vector(0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        )?;
        let xs = [Intersection::new(SQRT_2, floor)];
        let comps = Computations::prepare(&xs[0], &r, &xs)?;
        let color = w.shade_hit_helper(&comps, 5)?;

        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));

        Ok(())
    }
}
