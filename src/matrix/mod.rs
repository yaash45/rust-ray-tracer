#[allow(clippy::module_inception)]
mod matrix;
mod transformations;

pub use matrix::static_operations::inverse_4x4;
pub use matrix::Matrix;
pub use transformations::{
    rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transform,
    Transformable,
};
