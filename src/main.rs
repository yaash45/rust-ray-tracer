use anyhow::Result;
use raytracer::camera::Camera;
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::intersections::{hit, Ray};
use raytracer::lights::{lighting, Material, PointLight};
use raytracer::matrix::{
    rotation_x, rotation_y, rotation_z, scaling, translation, view_transform, Transformable,
};
use raytracer::patterns::{Solid, Striped};
use raytracer::shapes::{Intersect, Plane, Shape, Sphere, SurfaceNormal};
use raytracer::spatial::Tuple;
use raytracer::tick::{tick, Environment, Projectile};
use raytracer::world::World;
use std::f64::consts::PI;

#[allow(dead_code)]
/// Chapter 2 tick example
fn projectile_example() -> Result<()> {
    println!("It's tick tick time");

    let p = Tuple::point(0, 1, 0);
    let v = &Tuple::vector(1, 1.8, 0).normalize() * 11.25;
    let mut projectile = Projectile::new(p, v);

    let g = Tuple::vector(0, -0.1, 0);
    let w = Tuple::vector(-0.01, 0, 0);
    let environment = Environment::new(g, w);

    let width = 900;
    let height = 550;
    let mut canvas = Canvas::new(width, height);

    while projectile.position.get_y() >= 0.0 {
        let x = projectile.position.get_x();
        let y = projectile.position.get_y();

        if !(0.0..900.0).contains(&x) || !(0.0..500.0).contains(&(y)) {
            continue;
        }

        canvas.write_pixel(x as usize, height - y as usize, Color::red())?;
        projectile = tick(&environment, projectile);
    }

    write_canvas_to_file("./projectile.ppm", &canvas);

    Ok(())
}

#[allow(dead_code)]
/// Chapter 4 analog clock example
fn analog_clock() -> Result<()> {
    let height = 500;
    let width = 500;
    let mut canvas = Canvas::new(width, height);
    let p = Tuple::point(0, 200, 0);

    let mut angle = 0_f64;

    while angle < 2.0 * PI {
        let transform = (&translation(250, 250, 0) * &rotation_z(angle))?;
        let current_pos = &transform * &p;
        let cur_y = height - current_pos.get_y() as usize;
        let cur_x = current_pos.get_x() as usize;

        if (0..height).contains(&cur_y) && (0..width).contains(&cur_x) {
            canvas.write_pixel(cur_x, cur_y, Color::new(1, 1, 1))?;
        }

        angle += PI / 6.0;
    }

    write_canvas_to_file("./analog_clock.ppm", &canvas);

    Ok(())
}

#[allow(dead_code)]
fn cast_rays_on_sphere_2d() -> Result<()> {
    let canvas_pixels = 100;
    let height = canvas_pixels;
    let width = canvas_pixels;
    let ray_origin = Tuple::point(0, 0, -5);

    let wall_z = 10.0;
    let wall_size = 7.0; // max y
    let half = wall_size / 2.0;
    let pixel_size = wall_size / canvas_pixels as f64;

    let mut canvas = Canvas::new(height, width);

    let mut s = Sphere::default();
    s.transform_matrix = (&rotation_z(PI / 4.0) * &scaling(0.5, 1, 1))?;

    for y in 0..(height - 1) {
        let world_y = half - (y as f64 * pixel_size);

        for x in 0..(width - 1) {
            let world_x = -half + (x as f64 * pixel_size);

            let position_on_canvas = Tuple::point(world_x, world_y, wall_z);
            let direction = (&position_on_canvas - &ray_origin).normalize();

            let ray = Ray::new(ray_origin, direction)?;

            if hit(s.intersect(&ray)?).is_some() {
                canvas.write_pixel(x, y, Color::red())?;
            }
        }
    }

    write_canvas_to_file("./cast_rays.ppm", &canvas);

    Ok(())
}

#[allow(dead_code)]
fn cast_rays_on_sphere_3d() -> Result<()> {
    println!("It's time to render your first 3D sphere!");

    let canvas_pixels = 100;
    let height = canvas_pixels;
    let width = canvas_pixels;
    let ray_origin = Tuple::point(0, 0, -5);

    let wall_z = 10.0;
    let wall_size = 7.0; // max y
    let half = wall_size / 2.0;
    let pixel_size = wall_size / canvas_pixels as f64;

    let mut canvas = Canvas::new(height, width);

    let mut s = Sphere::default();
    s.material
        .set_pattern(Solid::from(Color::new(1, 1, 1)).into());

    let light_position = Tuple::point(-10, 10, -10);
    let light_color = Color::new(1, 0, 0);
    let light = PointLight::new(light_position, light_color)?;

    for y in 0..(width - 1) {
        let world_y = half - (y as f64 * pixel_size);

        for x in 0..(height - 1) {
            let world_x = -half + (x as f64 * pixel_size);

            let position_on_canvas = Tuple::point(world_x, world_y, wall_z);
            let direction = (&position_on_canvas - &ray_origin).normalize();

            let ray = Ray::new(ray_origin, direction)?;

            let cur_hit = hit(s.intersect(&ray)?);

            if cur_hit.is_some() {
                let point = ray.position(cur_hit.unwrap().t);
                let normal = s.normal_at(&point)?;
                let eye = -ray.direction;
                let color = lighting(
                    &s.material,
                    &Shape::Sphere(s),
                    &light,
                    &point,
                    &eye,
                    &normal,
                    false,
                )?; // placeholder until shadows are accounted for

                canvas.write_pixel(x, y, color)?;
            }
        }
    }

    write_canvas_to_file("./cast_rays3d.ppm", &canvas);

    Ok(())
}

#[allow(dead_code)]
fn render_a_world(vsize: usize, hsize: usize) -> Result<()> {
    let mut floor_material = Material::default();
    floor_material.set_pattern(Solid::from(Color::new(0.17, 0.4, 0.925)).into());
    floor_material.set_specular(0.0);
    let floor = Plane::new(translation(0, 0.4, 0), floor_material);

    let mut left_wall_transform = (&translation(0, 0, 5) * &rotation_y(-PI / 4.0))?;
    left_wall_transform = (&left_wall_transform * &rotation_x(PI / 2.0))?;
    left_wall_transform = (&left_wall_transform * &scaling(10, 0.01, 10))?;
    let mut left_wall_material = floor_material;
    left_wall_material.set_pattern(Solid::from(Color::new(1, 0.9, 0.9)).into());
    let left_wall = Plane::new(left_wall_transform, left_wall_material);

    let mut right_wall_transform = (&translation(0, 0, 5) * &rotation_y(PI / 4.0))?;
    right_wall_transform = (&right_wall_transform * &rotation_x(PI / 2.0))?;
    right_wall_transform = (&right_wall_transform * &scaling(10, 0.01, 10))?;
    let right_wall_material = left_wall_material;
    let right_wall = Plane::new(right_wall_transform, right_wall_material);

    let mut middle_material = Material::default();
    // middle_material.set_color(Color::new(0.1, 1, 0.5));
    middle_material.set_pattern(
        Striped::new(
            Color::white(),
            Color::blue(),
            (&scaling(0.1, 0.1, 0.1) * &rotation_y(PI / 4.0))?,
        )
        .into(),
    );
    middle_material.set_diffuse(0.7);
    middle_material.set_specular(0.3);
    let middle = Sphere::new(
        (&translation(-0.5, 1, 0.5) * &scaling(1.5, 1.5, 1.5))?,
        middle_material,
    );

    let mut right_material = Material::default();
    right_material.set_pattern(Solid::from(Color::new(0.5, 1, 0.1)).into());
    right_material.set_diffuse(0.7);
    right_material.set_specular(0.3);
    let right = Sphere::new(
        (&translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5))?,
        right_material,
    );

    let mut left_material = Material::default();
    left_material.set_pattern(Solid::from(Color::new(1, 0.8, 0.1)).into());
    left_material.set_diffuse(0.7);
    left_material.set_specular(0.3);
    let left = Sphere::new(
        (&translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33))?,
        left_material,
    );

    let light_source = PointLight::new(Tuple::point(-10, 10, -10), Color::new(1, 1, 1))?;

    let mut world = World::empty();
    world.set_light(Some(light_source));
    world.add_object(Shape::Plane(floor));
    world.add_object(Shape::Plane(left_wall));
    world.add_object(Shape::Plane(right_wall));
    world.add_object(Shape::Sphere(middle));
    world.add_object(Shape::Sphere(left));
    world.add_object(Shape::Sphere(right));

    let mut camera = Camera::new(hsize, vsize, PI / 3.0);
    camera.set_transform(view_transform(
        &Tuple::point(0, 1.5, -5),
        &Tuple::point(0, 1, 0),
        &Tuple::point(0, 1, 0),
    ));

    let canvas = camera.render(&world)?;

    write_canvas_to_file("./chapter7render.ppm", &canvas);

    Ok(())
}

fn write_canvas_to_file(filename: &str, canvas: &Canvas) {
    std::fs::write(
        filename,
        canvas.to_ppm().expect("could not convert canvas to PPM"),
    )
    .expect("Cannot write to file");
}

fn main() -> Result<()> {
    // Projectile example from chapter 2
    // projectile_example()?;

    // analog clock example from chapter 4
    // analog_clock()?;

    // cast rays on sphere example from chapter 5
    // cast_rays_on_sphere_2d()?;

    // cast rays on a sphere example from chapter 6
    // cast_rays_on_sphere_3d()?;

    // render a world from chapter 7, etc.
    render_a_world(100, 100)?;

    Ok(())
}
