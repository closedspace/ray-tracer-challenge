mod features;
use std::fs::File;
use std::io::Write;

use crate::features::canvas::Canvas;
use crate::features::color::Color;
use crate::features::tuple::Tuple;

fn main() {
    println!("Hello, world!");

    //add a projectile and an environment
    // a projectile consists of a position and a velocity
    // an environment has gravity and wind
    let mut projectile = (Tuple::point(0.0, 1.0, 0.0), Tuple::vector(1.0, 1.0, 0.0).normalize() * 11.25);
    let environment = (Tuple::vector(0.0, -0.1, 0.0), Tuple::vector(-0.01, 0.0, 0.0));

    // add a tick function
    fn tick(env: (Tuple, Tuple), proj: (Tuple, Tuple)) -> (Tuple, Tuple) {
        let position = proj.0 + proj.1;
        let velocity = proj.1 + env.0 + env.1;
        (position, velocity)
    }

    // add a canvas
    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1.0, 1.0, 0.0);

    // add a loop that simulates the projectile's flight
    // until it hits the ground or leaves the canvas
    while projectile.0.y > 0.0 && projectile.0.x < canvas.width as f64 {
        let x = projectile.0.x as usize;
        let y = canvas.height - projectile.0.y as usize;
        canvas.write_pixel(x, y, color);
        // make the dots thicker
        canvas.write_pixel(x + 1, y, color);
        canvas.write_pixel(x, y + 1, color);
        canvas.write_pixel(x + 1, y + 1, color);
        projectile = tick(environment, projectile);
    }

    canvas.canvas_to_file("projectile.ppm");
}
