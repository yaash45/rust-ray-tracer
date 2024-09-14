#[allow(clippy::module_inception)]
mod matrix;
mod matrix_buggy;

pub use matrix::Matrix;
pub use matrix_buggy::MatrixBuggy;
