mod objects;
mod operations;
mod ray;

pub use objects::{Intersection, Object, Sphere, SurfaceNormal};
pub use operations::{hit, reflect, transform_ray};
pub use ray::Ray;
