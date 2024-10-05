mod objects;
mod operations;
mod ray;

pub use objects::Intersection;
pub use objects::Object;
pub use objects::Sphere;
pub use operations::{hit, transform_ray};
pub use ray::Ray;
