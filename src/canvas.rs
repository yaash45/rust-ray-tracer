use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Canvas {
    inner: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    fn calculate_size(width: usize, height: usize) -> usize {
        width * height
    }

    fn map_index(width: usize, i: usize, j: usize) -> usize {
        (width * i) + j
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: vec![Color::default(); Self::calculate_size(width, height)],
        }
    }

    pub fn pixel_at(&self, i: usize, j: usize) -> &Color {
        &self.inner[Self::map_index(self.width, i, j)]
    }

    pub fn write_pixel(&mut self, i: usize, j: usize, color: Color) {
        self.inner[Self::map_index(self.width, i, j)] = color
    }

    pub fn fill_canvas(&mut self, color: Color) {
        self.inner = vec![color; Self::calculate_size(self.width, self.height)];
    }

    fn get_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    fn build_ppm_body(&self) -> String {
        let mut pixels = String::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let scaled_color_tuple = self.pixel_at(i, j).get_255_scaled_tuple();
                let scaled_color_string = format!(
                    "{} {} {}",
                    scaled_color_tuple.0, scaled_color_tuple.1, scaled_color_tuple.2
                );

                pixels.push_str(scaled_color_string.as_str());
                pixels.push(' ');
            }
            pixels.push('\n');
        }

        pixels
    }

    pub fn to_ppm(&self) -> String {
        let header = self.get_ppm_header();
        let pixels = self.build_ppm_body();
        header + &pixels
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;
    use crate::color::Color;

    #[test]
    fn create_canvas() {
        let w = 10;
        let h = 20;
        let canvas = Canvas::new(w, h);
        let black = Color::default();

        for i in 0..h {
            for j in 0..w {
                assert_eq!(canvas.pixel_at(i, j), &black)
            }
        }
    }

    #[test]
    fn write_to_canvas() {
        let w = 10;
        let h = 20;
        let red = Color::new(1, 0, 0);
        let mut canvas = Canvas::new(w, h);

        canvas.write_pixel(2, 3, red.clone());
        assert_eq!(canvas.pixel_at(2, 3), &red);
    }

    #[test]
    fn to_ppm_header() {
        let w = 3;
        let h = 2;
        let mut canvas = Canvas::new(w, h);

        canvas.fill_canvas(Color::new(1, 0.8, 0.6));
        canvas.to_ppm();
    }
}
