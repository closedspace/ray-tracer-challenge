use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    // values, rows, cols
}

impl Matrix {
    pub fn new() -> Self {
        // TODO
    }

    pub fn identify() -> Self {
        // TODO
    }
}

impl Add for Matrix {
    // TODO
}

impl Sub for Matrix {
    // TODO
}

impl Mul for Matrix {
    // TODO
}

// TODO: rewrite all instances of vec using tuples
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2x2_matrix() {
        let data = vec![vec![1, 2], vec![3, 4]];
        let m = Matrix::new(2, 2, data);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 2);
        assert_eq!(m.data, data);
    }

    #[test]
    fn test_3x3_matrix() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let m = Matrix::new(3, 3, data);
        assert_eq!(m.rows, 3);
        assert_eq!(m.cols, 3);
        assert_eq!(m.data, data);
    }

    #[test]
    fn test_4x4_matrix() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]];
        let m = Matrix::new(4, 4, data);
        assert_eq!(m.rows, 4);
        assert_eq!(m.cols, 4);