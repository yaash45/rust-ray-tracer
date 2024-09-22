#[allow(clippy::module_inception)]
mod matrix;
mod transformations;

pub use matrix::static_operations::inverse_4x4;
pub use matrix::Matrix;
pub use transformations::{scaling, translation};
