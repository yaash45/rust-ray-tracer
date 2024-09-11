use crate::color::Color;
use anyhow::{Error, Result};

#[derive(Clone, Debug)]
/// Representing a rectangular grid of pixels, that hold a
/// single [Color] each
pub struct Canvas {
    inner: Vec<Color>,
    /// The measure of the Width of the canvas (in pixels)
    pub width: usize,
    /// The measure of the Height of the canvas (in pixels)
    pub height: usize,
}

impl Canvas {
    /// Build a new [Canvas] of with the number of width
    /// and height pixels specified in the arguments.
    ///
    /// _Note: the canvas is initialized with all pixels set to color (0,0,0)_
    ///
    /// ```
    /// use raytracer::canvas::Canvas;
    ///
    /// let canvas = Canvas::new(10, 20);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: vec![Color::default(); width * height],
        }
    }

    /// Helper method to generate a consistent out-of-bounds error message
    fn get_out_of_bounds_error_message(&self, x: &usize, y: &usize) -> String {
        format!(
            "Index ({}, {}) is out-of-bounds for \
        this canvas of size ({}, {})",
            x, y, self.width, self.height
        )
    }

    /// Helper method to check if a queried index is out-of-bounds in our [Canvas]
    fn is_out_of_bounds(&self, x: &usize, y: &usize) -> bool {
        !(0..self.width).contains(x) || !(0..self.height).contains(y)
    }

    /// Gets the [Color] at the pixel coordinates `(x,y)`.
    /// This returns an error, if the requested pixel is out-of-bounds.
    ///
    /// ```
    /// use raytracer::{canvas::Canvas, color::Color};
    ///
    /// let canvas = Canvas::new(4,4);
    ///
    /// // obtaining a pixel's color can be done like this
    /// let pixel_color = canvas.pixel_at(3, 1);
    ///
    /// match pixel_color {
    ///     Ok(c) => println!("Success! {}", c),
    ///     Err(_e) => panic!("it shouldn't be an error"),
    /// }
    ///
    /// let invalid_color = canvas.pixel_at(400,0);
    ///
    /// match invalid_color {
    ///     Ok(_c) => panic!("this should be an out-of-bounds error"),
    ///     Err(e) => println!("out of bounds")
    /// }
    /// ```
    pub fn pixel_at(&self, x: usize, y: usize) -> Result<&Color> {
        if self.is_out_of_bounds(&x, &y) {
            Err(Error::msg(self.get_out_of_bounds_error_message(&x, &y)))
        } else {
            Ok(&self.inner[self.map_index(x, y)])
        }
    }

    /// Writes the given [Color] at the pixel coordinate `(x,y)`,
    /// This returns an error, if the pixel written to is out-of-bounds
    ///
    /// ```
    /// use raytracer::{canvas::Canvas, color::Color};
    ///
    /// let mut canvas = Canvas::new(4,4);
    ///
    /// // A pixel can be written to like this:
    /// let ok_result = canvas.write_pixel(3, 1, Color::red());
    ///
    /// match ok_result {
    ///     Ok(()) => println!("Success!"),
    ///     Err(_e) => panic!("it shouldn't be an error"),
    /// }
    ///
    /// let err_result = canvas.write_pixel(400,0, Color::blue());
    ///
    /// match err_result {
    ///     Ok(_c) => panic!("this should be an out-of-bounds error"),
    ///     Err(e) => println!("out of bounds")
    /// }
    /// ```
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<()> {
        if self.is_out_of_bounds(&x, &y) {
            Err(Error::msg(self.get_out_of_bounds_error_message(&x, &y)))
        } else {
            let map_index = self.map_index(x, y);
            self.inner[map_index] = color;
            Ok(())
        }
    }

    /// Convenient way to color the entire canvas with the same [Color]
    ///
    /// ```
    /// use raytracer::{canvas::Canvas, color::Color};
    ///
    /// let mut canvas = Canvas::new(10, 10);
    ///
    /// // This should color the entire canvas red
    /// canvas.fill(Color::red());
    /// ```
    pub fn fill(&mut self, color: Color) {
        self.inner = vec![color; self.width * self.height];
    }

    /// Builds a PPM header string
    fn get_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    /// Builds the body of the PPM file by applying our canvas
    /// into a valid PPM format string
    fn build_ppm_body(&self) -> Result<String> {
        let mut pixels = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let scaled_color_tuple = self.pixel_at(x, y)?.get_255_scaled_tuple();
                let scaled_color_string = format!(
                    "{} {} {}",
                    scaled_color_tuple.0, scaled_color_tuple.1, scaled_color_tuple.2
                );

                pixels.push_str(scaled_color_string.as_str());

                if x < self.width - 1 {
                    pixels.push(' ');
                }
            }
            pixels.push('\n');
        }

        Ok(pixels)
    }

    /// Converts the current canvas to a [String] that is formatted
    /// in valid PPM format. This can be written to a PPM file, and
    /// opened with most major image viewing software (like Preview, Gimp, etc.)
    ///
    /// ```
    /// use raytracer::{canvas::Canvas, color::Color};
    ///
    /// let mut canvas = Canvas::new(2,2);
    /// canvas.write_pixel(0,0, Color::red());
    ///
    /// let ppm_string_result = canvas.to_ppm();
    ///
    /// let ppm_string = match ppm_string_result {
    ///                     Ok(ppm) => ppm,
    ///                     Err(e) => panic!("this shouldn't happen"),
    ///                 };
    ///
    /// let mut ppm_lines = ppm_string.lines();
    ///
    /// // Header (lines 1-3) must look like this:
    /// assert_eq!(Some("P3"), ppm_lines.next());
    /// assert_eq!(Some("2 2"), ppm_lines.next());
    /// assert_eq!(Some("255"), ppm_lines.next());
    ///
    /// // Body (lines 4-6) must look like this:
    /// assert_eq!(Some("255 0 0 0 0 0"), ppm_lines.next());
    /// assert_eq!(Some("0 0 0 0 0 0"), ppm_lines.next());
    ///
    /// // ppm must end in a newline
    /// assert!(ppm_string.ends_with("\n"));
    ///
    /// // In totality, it should look something like this:
    /// // """
    /// // P3
    /// // 2 2
    /// // 255
    /// // 255 0 0 0 0 0
    /// // 0 0 0 0 0 0
    /// //
    /// // """
    /// ```
    pub fn to_ppm(&self) -> Result<String> {
        let header = self.get_ppm_header();
        let pixels = self.build_ppm_body()?;
        Ok(header + &pixels)
    }

    fn map_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;
    use crate::color::Color;
    use anyhow::Result;

    #[test]
    fn create_canvas() -> Result<()> {
        let w = 10;
        let h = 20;
        let canvas = Canvas::new(w, h);
        let black = Color::default();

        for y in 0..h {
            for x in 0..w {
                assert_eq!(canvas.pixel_at(x, y)?, &black)
            }
        }
        Ok(())
    }

    #[test]
    fn write_to_canvas() -> Result<()> {
        let w = 10;
        let h = 20;
        let red = Color::new(1, 0, 0);
        let mut canvas = Canvas::new(w, h);

        canvas.write_pixel(2, 3, red.clone())?;
        assert_eq!(canvas.pixel_at(2, 3)?, &red);
        Ok(())
    }

    #[test]
    fn to_ppm() -> Result<()> {
        let w = 5;
        let h = 3;

        let c1 = Color::from((1.5, 0, 0));
        let c2 = Color::from((0, 0.5, 0));
        let c3 = Color::from((-0.5, 0, 1));

        let mut canvas = Canvas::new(w, h);
        canvas.write_pixel(0, 0, c1)?;
        canvas.write_pixel(2, 1, c2)?;
        canvas.write_pixel(4, 2, c3)?;

        let ppm = canvas.to_ppm()?;

        let mut lines = ppm.lines();

        // Expect lines 1-3 to contain the PPM header
        assert_eq!(Some("P3"), lines.next());
        assert_eq!(Some("5 3"), lines.next());
        assert_eq!(Some("255"), lines.next());

        // Expect lines 4-7 to contain our color pixels
        assert_eq!(Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"), lines.next());
        assert_eq!(Some("0 0 0 0 0 0 0 127 0 0 0 0 0 0 0"), lines.next());
        assert_eq!(Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), lines.next());

        // Check that our ppm ends with a newline character
        assert!(ppm.ends_with("\n"));

        Ok(())
    }
}
