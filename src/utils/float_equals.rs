const EPSILON: f64 = 0.00001;

/// Helper function to properly compare the equality
/// of two 64-bit precision floating point numbers.
///
/// This accounts for there being an error of at most
/// [EPSILON] in difference between `a` and `b`
pub fn float_equals(a: &f64, b: &f64) -> bool {
    (a - b).abs() < EPSILON
}
