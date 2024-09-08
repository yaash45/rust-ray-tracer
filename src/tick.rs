use crate::spatial::Tuple;
pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }
}

pub fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile::new(
        proj.position + proj.velocity,
        proj.velocity + env.gravity + env.wind,
    )
}
