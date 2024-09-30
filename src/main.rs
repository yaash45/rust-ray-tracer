use std::f64::consts::PI;

use anyhow::Result;
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::matrix::{rotation_z, translation};
use raytracer::spatial::Tuple;
use raytracer::tick::{tick, Environment, Projectile};

/// Chapter 2 tick example
fn projectile_example() -> Result<()> {
    println!("It's tick tick time");

    let p = Tuple::new_point(0, 1, 0);
    let v = &Tuple::new_vector(1, 1.8, 0).normalize() * 11.25;
    let mut projectile = Projectile::new(p, v);

    let g = Tuple::new_vector(0, -0.1, 0);
    let w = Tuple::new_vector(-0.01, 0, 0);
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

    std::fs::write(
        "./projectile.ppm",
        canvas.to_ppm().expect("could not convert to ppm"),
    )
    .expect("Cannot write");

    Ok(())
}

/// Chapter 4 analock clock example
fn analog_clock() -> Result<()> {
    let height = 500;
    let width = 500;
    let mut canvas = Canvas::new(width, height);
    let p = Tuple::new_point(0, 200, 0);

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

    std::fs::write(
        "./analog_clock.ppm",
        canvas.to_ppm().expect("could not convert to ppm"),
    )
    .expect("Cannot write");

    Ok(())
}

fn main() -> Result<()> {
    // Projectile example from chapter 2
    projectile_example()?;

    // analog clock example from chapter 4
    analog_clock()?;

    Ok(())
}
