use crate::utils::float_equals;
use std::fmt::Display;
use std::ops;

#[derive(Debug, Clone, Copy, PartialOrd)]
/// Representation of colors using RGB values
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    /// Create new colors using this constructor method. It acecpts numerical
    /// values for the Red, Green, and Blue values that define the color.
    ///
    /// ```
    /// use raytracer::color::Color;
    ///
    /// let black = Color::new(0,0,0);
    /// assert_eq!(black.red, 0.0);
    /// assert_eq!(black.green, 0.0);
    /// assert_eq!(black.blue, 0.0);
    ///
    /// let red = Color::new(1,0,0);
    /// assert_eq!(red.red, 1.0);
    ///
    /// let color = Color::new(0.2, 0.6, 1);
    /// println!("{}", color);
    /// ```
    pub fn new(red: impl Into<f64>, green: impl Into<f64>, blue: impl Into<f64>) -> Self {
        Color {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
        }
    }

    /// Quick shortcut method to create the color Black (0,0,0)
    ///
    /// ```
    /// use raytracer::color::Color;
    ///
    /// let b = Color::black();
    /// assert_eq!(b.red, 0.0);
    /// assert_eq!(b.green, 0.0);
    /// assert_eq!(b.blue, 0.0);
    /// ```
    pub fn black() -> Self {
        Self::default()
    }

    /// Quick shortcut method to create the color Red (1,0,0)
    ///
    /// ```
    /// use raytracer::color::Color;
    ///
    /// let r = Color::red();
    /// assert_eq!(r.red, 1.0);
    /// assert_eq!(r.green, 0.0);
    /// assert_eq!(r.blue, 0.0);
    /// ```
    pub fn red() -> Self {
        Self::new(1, 0, 0)
    }

    /// Quick shortcut method to create the color Green (0,1,0)
    ///
    /// ```
    /// use raytracer::color::Color;
    ///
    /// let g = Color::green();
    /// assert_eq!(g.red, 0.0);
    /// assert_eq!(g.green, 1.0);
    /// assert_eq!(g.blue, 0.0);
    /// ```
    pub fn green() -> Self {
        Self::new(0, 1, 0)
    }

    /// Quick shortcut method to create the color Blue (0,0,1)
    ///
    /// ```
    /// use raytracer::color::Color;
    ///
    /// let b = Color::blue();
    /// assert_eq!(b.red, 0.0);
    /// assert_eq!(b.green, 0.0);
    /// assert_eq!(b.blue, 1.0);
    /// ```
    pub fn blue() -> Self {
        Self::new(0, 0, 1)
    }

    /// The `hadamard_product` is an operation that yields a
    /// new [Color] obtained by multiplying the individual
    /// elements of the two input [Color]s.
    ///
    /// ```
    /// use raytracer::color::Color;
    ///
    /// let c1 = Color::from((1, 0.2, 0.4));
    /// let c2 = Color::from((0.9, 1, 0.1));
    /// assert_eq!(c1.hadamard_product(&c2), Color::from((0.9, 0.2, 0.04)));
    /// assert_eq!(c1.hadamard_product(&c2), &c1 * &c2);
    /// ```
    pub fn hadamard_product(&self, other: &Color) -> Self {
        self * other
    }

    /// Scales the color into integer values so the RGB components
    /// fall in the range \[0, 255\], and returns them as a simple tuple
    ///
    /// ```
    /// use raytracer::color::Color;
    ///
    /// let c1 = Color::from((1, 0.2, 0.4));
    /// assert_eq!(c1.get_255_scaled_tuple(), (255, 51, 102));
    /// ```
    pub fn get_255_scaled_tuple(&self) -> (usize, usize, usize) {
        let r = ((self.red * 255.0) as usize).clamp(0, 255);
        let g = ((self.green * 255.0) as usize).clamp(0, 255);
        let b = ((self.blue * 255.0) as usize).clamp(0, 255);
        (r, g, b)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float_equals(&self.red, &other.red)
            && float_equals(&self.green, &other.green)
            && float_equals(&self.blue, &other.blue)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl From<&Color> for String {
    fn from(value: &Color) -> Self {
        format!("{} {} {}", value.red, value.blue, value.green)
    }
}

impl<T, U, G> From<(T, U, G)> for Color
where
    T: Into<f64>,
    U: Into<f64>,
    G: Into<f64>,
{
    fn from(value: (T, U, G)) -> Self {
        Self::new(value.0.into(), value.1.into(), value.2.into())
    }
}

impl ops::Add<&Color> for &Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl ops::Sub<&Color> for &Color {
    type Output = Color;

    fn sub(self, rhs: &Color) -> Self::Output {
        Color::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl<T> ops::Mul<T> for &Color
where
    T: Into<f64> + Clone + Copy,
{
    type Output = Color;

    fn mul(self, rhs: T) -> Self::Output {
        Color::new(
            self.red * rhs.into(),
            self.green * rhs.into(),
            self.blue * rhs.into(),
        )
    }
}

impl<T> ops::Mul<T> for Color
where
    T: Into<f64> + Clone + Copy,
{
    type Output = Color;

    fn mul(self, rhs: T) -> Self::Output {
        Color::new(
            self.red * rhs.into(),
            self.green * rhs.into(),
            self.blue * rhs.into(),
        )
    }
}

impl<T> ops::MulAssign<T> for Color
where
    T: Into<f64> + Clone + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.red = rhs.into();
        self.green = rhs.into();
        self.blue = rhs.into();
    }
}

impl ops::Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl ops::Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.red / rhs, self.green / rhs, self.blue / rhs)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "| R: {:.1} |", self.red)?;
        writeln!(f, "| G: {:.1} |", self.green)?;
        writeln!(f, "| B: {:.1} |", self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn color_getters() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(color.red, -0.5);
        assert_eq!(color.green, 0.4);
        assert_eq!(color.blue, 1.7);

        let color = Color::from((0.2, 0, 1));
        assert_eq!(color.red, 0.2);
        assert_eq!(color.green, 0.0);
        assert_eq!(color.blue, 1.0);
    }

    #[test]
    fn color_operations() {
        let c_a = Color::from((0.9, 0.6, 0.75));
        let c_b = Color::from((0.7, 0.1, 0.25));

        assert_eq!(c_a + c_b, Color::from((1.6, 0.7, 1.0)));

        // I am expressing the expected color this way because
        // rust has a small deviation in the result of the f64
        // subtraction
        assert_eq!(c_a - c_b, Color::from((0.2, 0.5, 0.50)));

        let c_c = Color::from((0.2, 0.3, 0.4));
        let scalar = 2;

        assert_eq!(c_c * scalar, Color::from((0.4, 0.6, 0.8)));

        let c_d = Color::from((1, 0.2, 0.4));
        let c_e = Color::from((0.9, 1, 0.1));

        assert_eq!(c_d * c_e, Color::from((0.9, 0.2, 0.04)));
        assert_eq!(c_d.hadamard_product(&c_e), Color::from((0.9, 0.2, 0.04)));
    }

    #[test]
    fn get_255_scaled_tuple() {
        let c1 = Color::from((1, 0.2, 0.4));
        assert_eq!(c1.get_255_scaled_tuple(), (255, 51, 102));
    }
}
