use std::{fs::File, io::Write};

use super::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Color::new(0.0, 0.0, 0.0));
            }
            pixels.push(row);
        }
        Self { width, height, pixels }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }
    // make sure to split the lines at 70 characters
    pub fn canvas_to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut line_length = 0;
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                let r = (pixel.red * 255.0).round() as u8;
                let g = (pixel.green * 255.0).round() as u8;
                let b = (pixel.blue * 255.0).round() as u8;
                let line = format!("{} {} {} ", r, g, b);
                line_length += line.len();
                if line_length > 70 {
                    ppm.push_str("\n");
                    line_length = line.len();
                }
                ppm.push_str(&line);
            }
            ppm.push_str("\n");
        }
        ppm
    }

    pub fn canvas_to_file(&self, filename: &str) {
        let ppm = self.canvas_to_ppm();
        let mut file = File::create(filename).unwrap();
        file.write_all(ppm.as_bytes()).unwrap();
    }
        
}

// write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for row in c.pixels {
            for pixel in row {
                assert_eq!(pixel, Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn canvas_to_ppm() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.canvas_to_ppm();
        let expected = "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n";
        assert_eq!(ppm, expected);
    }

    // splitlong lines in ppm 
    #[test]
    fn canvas_to_ppm_split_lines() {
        let mut c = Canvas::new(10, 2);
        let c1 = Color::new(1.0, 0.8, 0.6);
        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, c1);
            }
        }
        let ppm = c.canvas_to_ppm();
        let expected = "P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n";
        assert_eq!(ppm, expected);
    }

}
