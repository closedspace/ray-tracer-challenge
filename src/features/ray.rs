use super::tuple::Tuple;


pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple, 
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        if !origin.is_point() {
            panic!("Origin must be a point")
        }

        if !direction.is_vector() {
            panic!("Direction must be a vector")
        }

        Self {
            origin,
            direction
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }
}