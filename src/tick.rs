use crate::tuples::SpatialTuple;
pub struct Environment {
    gravity: SpatialTuple,
    wind: SpatialTuple,
}

#[derive(Debug)]
pub struct Projectile {
    pub position: SpatialTuple,
    velocity: SpatialTuple,
}

impl Environment {
    pub fn new(gravity: &SpatialTuple, wind: &SpatialTuple) -> Self {
        Self {
            gravity: gravity.clone(),
            wind: wind.clone(),
        }
    }
}

impl Projectile {
    pub fn new(position: &SpatialTuple, velocity: &SpatialTuple) -> Self {
        Self {
            position: position.clone(),
            velocity: velocity.clone(),
        }
    }
}

pub fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let env_velocity = &env.gravity + &env.wind;

    Projectile {
        position: &proj.position + &proj.velocity,
        velocity: &proj.velocity + &env_velocity,
    }
}
