use std::ops::Add;

use std::f64::EPSILON;

fn almost_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }

    fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

// TODO: IMPLEMENT SUBTRACTION, SCALAR MULTIPLICATION AND DIVISION, NEGATION, MAGNITUDE
// NORMALIZATION, DOT PRODUCT, CROSS PRODUCT,

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        almost_equal(self.x, other.x)
            && almost_equal(self.y, other.y)
            && almost_equal(self.z, other.z)
            && almost_equal(self.w, other.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w1_is_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.is_point(), true);
    }

    #[test]
    fn tuple_with_w0_is_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.is_vector(), true);
    }

    #[test]
    fn point_creates_tuples_with_w1() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuples_with_w0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }
    
    #[test]
    fn adding_two_vectors() {
        let a1 = Tuple::vector(3.0, -2.0, 5.0);
        let a2 = Tuple::vector(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, Tuple::vector(1.0, 1.0, 6.0));
    }

    #[test]
    fn adding_vector_and_point() {
        let p = Tuple::point(3.0, -2.0, 5.0);
        let v = Tuple::vector(-2.0, 3.0, 1.0);
        assert_eq!(p + v, Tuple::point(1.0, 1.0, 6.0));
    }

    #[test]
    fn adding_point_and_point() {
        let p1 = Tuple::point(3.0, -2.0, 5.0);
        let p2 = Tuple::point(-2.0, 3.0, 1.0);
        assert_eq!(p1 + p2, Tuple::new(1.0, 1.0, 6.0, 2.0));
    }

    #[test]
    fn tuples_with_same_values_are_equal() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let b = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a, b);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }
}
