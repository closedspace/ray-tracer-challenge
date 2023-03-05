mod features;

use std::f64::consts::PI;

use features::matrix::Matrix;

use crate::features::canvas::Canvas;
use crate::features::color::Color;
use crate::features::tuple::Tuple;

fn projectile_model() {
    let mut projectile = (Tuple::point(0.0, 1.0, 0.0), Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25);
    let environment = (Tuple::vector(0.0, -0.1, 0.0), Tuple::vector(-0.01, 0.0, 0.0));

    fn tick(env: (Tuple, Tuple), proj: (Tuple, Tuple)) -> (Tuple, Tuple) {
        let position = proj.0 + proj.1;
        let velocity = proj.1 + env.0 + env.1;
        (position, velocity)
    }

    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1.0, 1.0, 0.0);

    while projectile.0.y > 0.0 && projectile.0.x < canvas.width as f64 {
        let x = projectile.0.x as usize;
        let y = canvas.height - projectile.0.y as usize;
        canvas.write_pixel(x, y, color);
        canvas.write_pixel(x + 1, y, color);
        canvas.write_pixel(x, y + 1, color);
        canvas.write_pixel(x + 1, y + 1, color);
        projectile = tick(environment, projectile);
    }

    canvas.canvas_to_file("projectile.ppm");
}

fn draw_clock() {
    let mut canvas = Canvas::new(600, 600);
    let white_color = Color::new(1.0, 1.0, 1.0);
    let magenta_color = Color::new(1.0, 0.0, 1.0);

    let center = Tuple::point(canvas.width as f64 / 2.0, canvas.height as f64 / 2.0, 0.0);
    let translation = Matrix::translation(center.x, center.y, center.z);
    let rotation = Matrix::rotation_z(PI / 30.0);
    let mut transform = translation.clone() * rotation.clone();
    for index in 0..60 {
        let point = transform.clone() * Tuple::point(0.0, -(canvas.height as f64) / 3.0, 0.0);
        if index % 5 == 0 {
            canvas.write_pixel(point.x as usize, point.y as usize, white_color);
            canvas.write_pixel(point.x as usize + 1, point.y as usize, white_color);
            canvas.write_pixel(point.x as usize, point.y as usize + 1, white_color);
            canvas.write_pixel(point.x as usize + 1, point.y as usize + 1, white_color);
        } else {
            canvas.write_pixel(point.x as usize, point.y as usize, magenta_color);
            canvas.write_pixel(point.x as usize + 1, point.y as usize, magenta_color);
            canvas.write_pixel(point.x as usize, point.y as usize + 1, magenta_color);
            canvas.write_pixel(point.x as usize + 1, point.y as usize + 1, magenta_color); 
        } 
        
        transform = transform.clone() * rotation.clone();
    }
    canvas.canvas_to_file("clock.ppm"); 
}
fn main() {
    draw_clock();
}
