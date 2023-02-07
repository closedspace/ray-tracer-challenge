mod features;
use std::fs::File;
use std::io::Write;

use crate::features::canvas::Canvas;
use crate::features::color::Color;
fn main() {
    println!("Hello, world!");

let mut canvas = Canvas::new(10, 20);
canvas.write_pixel(5, 10, Color::new(1.0, 1.0, 0.0));
let ppm = canvas.canvas_to_ppm();

let mut file = File::create("canvas.ppm").unwrap();
file.write_all(ppm.as_bytes()).unwrap();

}
