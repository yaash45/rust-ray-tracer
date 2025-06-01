// TODO: Reconstruct the epsilon determination
// when running tests, due to lower precision
// float values provided by the textbook
pub const EPSILON: f64 = 2e-4;

/// Helper function to properly compare the equality
/// of two 64-bit precision floating point numbers.
///
/// This accounts for there being an error of at most
/// [EPSILON] in difference between `a` and `b`
pub fn float_equals(a: &f64, b: &f64) -> bool {
    (a - b).abs() < EPSILON
}
