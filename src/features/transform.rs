use crate::features::matrix::Matrix;

// Write struct Transform with a column matrix attribute
pub struct Transform {
    pub matrix: Matrix,
}

impl Transform {
    // Write a constructor that takes a column matrix as an argument
    pub fn new(matrix: Matrix) -> Self {
        Self { matrix }
    }

    // Write a method that returns the identity transform
    pub fn identity() -> Self {
        Self::new(Matrix::identity(3))
    }

    // Write a method that returns the translation transform
    pub fn translation(x: f64, y: f64) -> Self {
        let mut matrix = Matrix::identity(3);
        matrix.rows[0][2] = x;
        matrix.rows[1][2] = y;
        Self::new(matrix)
    }

    // Write a method that returns the scaling transform
    pub fn scaling(x: f64, y: f64) -> Self {
        let mut matrix = Matrix::identity(3);
        matrix.rows[0][0] = x;
        matrix.rows[1][1] = y;
        Self::new(matrix)
    }

    // Write a method that returns the rotation transform
    pub fn rotation(radians: f64) -> Self {
        let mut matrix = Matrix::identity(3);
        matrix.rows[0][0] = radians.cos();
        matrix.rows[0][1] = -radians.sin();
        matrix.rows[1][0] = radians.sin();
        matrix.rows[1][1] = radians.cos();
        Self::new(matrix)
    }

    // Write a method that returns the shearing transform
    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self {
        let mut matrix = Matrix::identity(3);
        matrix.rows[0][1] = x_y;
        matrix.rows[0][2] = x_z;
        matrix.rows[1][0] = y_x;
        matrix.rows[1][2] = y_z;
        matrix.rows[2][0] = z_x;
        matrix.rows[2][1] = z_y;
        Self::new(matrix)
    }

    // Write a method that returns the transform matrix
    pub fn matrix(&self) -> Matrix {
        self.matrix.clone()
    }

    // Write a method that returns the inverse of the transform
    pub fn inverse(&self) -> Self {
        let mut matrix = self.matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::tuple::Tuple;

    #[test]
    fn test_translation() {
        let transform = Transform::translation(5.0, -3.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform.matrix() * p, Tuple::point(2.0, 1.0, 5.0));
    }

    #[test]
    fn test_inverse_translation() {
        let transform = Transform::translation(5.0, -3.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(inv.matrix() * p, Tuple::point(-8.0, 7.0, 5.0));
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let transform = Transform::translation(5.0, -3.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(transform.matrix() * v, v);
    }

    #[test]
    fn test_scaling() {
        let transform = Transform::scaling(2.0, 3.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(transform.matrix() * p, Tuple::point(-8.0, 18.0, 24.0));
    }

    #[test]
    fn test_scaling_vector() {
        let transform = Transform::scaling(2.0, 3.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform.matrix() * v, Tuple::vector(-8.0, 18.0, 24.0));
    }

    #[test]
    fn test_inverse_scaling() {
        let transform = Transform::scaling(2.0, 3.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(inv.matrix() * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
}


// TODO: linear transformations
    // This can be done by multiplication of a matrix with a column vector
    // which represents a point.

// Scaling (generalize the matrix)
    // | sx 0  0 |
    // | 0  sy 0 |
    // | 0  0  1 |


// Rotations 
    // (2d)
    // | cos(θ) -sin(θ) 0 |
    // | sin(θ)  cos(θ) 0 |
    // | 0        0      1 |

    // Example 3d
    // | 1         0          0       |
    // | 0  cos(θ_x) -sin(θ_x)      0 |
    // | 0  sin(θ_x)  cos(θ_x)      0 |
    // | 0         0          0       |

// Translations
    // Example 2d
    // | 1 0 dx |
    // | 0 1 dy |
    // | 0 0 1  |

// Skew transformation
    // Horizontal skew example
    // | 1 tan(α) 0 |
    // | 0 1      0 |
    // | 0 0      1 |

    // Horizontal skew example
    // | 1      0 0 |
    // | tan(β) 1 0 |
    // | 0      0 1 |