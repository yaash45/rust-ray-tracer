use crate::tuples::SpatialTuple;
pub struct Environment {
    gravity: SpatialTuple,
    wind: SpatialTuple,
}

#[derive(Debug)]
pub struct Projectile {
    pub position: SpatialTuple,
    pub velocity: SpatialTuple,
}

impl Environment {
    pub fn new(gravity: SpatialTuple, wind: SpatialTuple) -> Self {
        Self { gravity, wind }
    }
}

impl Projectile {
    pub fn new(position: SpatialTuple, velocity: SpatialTuple) -> Self {
        Self { position, velocity }
    }
}

pub fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile::new(
        proj.position + proj.velocity,
        proj.velocity + env.gravity + env.wind,
    )
}
