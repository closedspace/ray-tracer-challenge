use std::f64::EPSILON;

pub fn almost_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}