mod canvas;
mod color;
mod spatial_identifier;
mod tick;
mod tuples;

use canvas::Canvas;
use color::Color;
use tick::{Environment, Projectile};
use tuples::SpatialTuple;

fn main() {
    println!("It's tick tick time");

    let p = SpatialTuple::new_point(0, 1, 0);
    let v = &SpatialTuple::new_vector(1, 1.8, 0).normalize() * 11.25;
    let mut projectile = Projectile::new(p, v);

    let g = SpatialTuple::new_vector(0, -0.1, 0);
    let w = SpatialTuple::new_vector(-0.01, 0, 0);
    let environment = Environment::new(g, w);

    let width = 900;
    let height = 550;
    let mut canvas = Canvas::new(width, height);

    let mut i = 0;

    while projectile.position.get_y() >= 0.0 {
        i += 1;

        let x = projectile.position.get_x();
        let y = projectile.position.get_y();

        if !(0.0..900.0).contains(&x) || !(0.0..500.0).contains(&(y)) {
            continue;
        }

        canvas.write_pixel(x as usize, height - y as usize, Color::red());
        projectile = tick::tick(&environment, projectile);
    }

    std::fs::write("./projectile.ppm", canvas.to_ppm()).expect("Cannot write");
}
