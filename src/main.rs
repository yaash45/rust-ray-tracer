mod canvas;
mod color;
mod spatial_identifier;
mod tuples;

use canvas::Canvas;
use color::Color;
use std::fs;

fn main() {
    let w = 10;
    let h = 2;
    let mut canvas = Canvas::new(w, h);

    canvas.fill_canvas(Color::new(1, 0.8, 0.6));
    canvas.write_pixel(0, 0, Color::default());

    let ppm = canvas.to_ppm();

    println!("{}", ppm);

    fs::write("./test1.ppm", ppm).expect("Unable to write");
}
