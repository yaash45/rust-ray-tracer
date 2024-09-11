use anyhow::Result;
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::spatial::Tuple;
use raytracer::tick::{tick, Environment, Projectile};

fn main() -> Result<()> {
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
