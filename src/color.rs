use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: impl Into<f64>, green: impl Into<f64>, blue: impl Into<f64>) -> Self {
        Color {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
        }
    }

    pub fn get_red(&self) -> f64 {
        self.red
    }

    pub fn get_green(&self) -> f64 {
        self.green
    }

    pub fn get_blue(&self) -> f64 {
        self.blue
    }

    pub fn hadamard_product(&self, other: &Color) -> Self {
        self * other
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
    T: Into<f64> + Clone,
{
    type Output = Color;

    fn mul(self, rhs: T) -> Self::Output {
        Color::new(
            self.red * rhs.clone().into(),
            self.green * rhs.clone().into(),
            self.blue * rhs.clone().into(),
        )
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

impl ops::Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.red / rhs, self.green / rhs, self.blue / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn color_getters() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(color.get_red(), -0.5);
        assert_eq!(color.get_green(), 0.4);
        assert_eq!(color.get_blue(), 1.7);

        let color = Color::from((0.2, 0, 1));
        assert_eq!(color.get_red(), 0.2);
        assert_eq!(color.get_green(), 0.0);
        assert_eq!(color.get_blue(), 1.0);
    }

    #[test]
    fn color_operations() {
        let c_a = Color::from((0.9, 0.6, 0.75));
        let c_b = Color::from((0.7, 0.1, 0.25));

        assert_eq!(&c_a + &c_b, Color::from((1.6, 0.7, 1.0)));

        // I am expressing the expected color this way because
        // rust has a small deviation in the result of the f64
        // subtraction
        assert_eq!(
            &c_a - &c_b,
            Color::from(((0.9 - 0.7), (0.6 - 0.1), (0.75 - 0.25)))
        );

        let c_c = Color::from((0.2, 0.3, 0.4));
        let scalar = 2;

        assert_eq!(&c_c * scalar, Color::from((0.4, 0.6, 0.8)));

        let c_d = Color::from((1, 0.2, 0.4));
        let c_e = Color::from((0.9, 1, 0.1));

        assert_eq!(&c_d * &c_e, Color::from((0.9, 0.2, (0.4 * 0.1))));
        assert_eq!(
            c_d.hadamard_product(&c_e),
            Color::from((0.9, 0.2, (0.4 * 0.1)))
        );
    }
}
