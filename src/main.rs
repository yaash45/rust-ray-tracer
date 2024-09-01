mod canvas;
mod color;
mod spatial_identifier;
mod tick;
mod tuples;

use tick::{Environment, Projectile};
use tuples::SpatialTuple;

fn main() {
    println!("It's tick tick time");

    let p = SpatialTuple::new_point(0, 1, 1);
    let v = SpatialTuple::new_vector(1, 1, 1).normalize();
    let mut projectile = Projectile::new(&p, &v);

    let g = SpatialTuple::new_vector(0, -0.1, -0.2);
    let w = SpatialTuple::new_vector(-0.01, 0, -0.1);
    let environment = Environment::new(&g, &w);

    let mut i = 0;

    while projectile.position.get_y() >= 0.0 {
        i += 1;
        println!(
            "position after iteration {} = ({},{},{})",
            i,
            projectile.position.get_x(),
            projectile.position.get_y(),
            projectile.position.get_z()
        );

        projectile = tick::tick(&environment, &projectile);
    }
}
