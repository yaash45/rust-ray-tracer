mod objects;
mod operations;
mod ray;

pub use objects::{Object, Sphere, SurfaceNormal};
pub use operations::{hit, reflect, transform_ray, Intersection};
pub use ray::Ray;
