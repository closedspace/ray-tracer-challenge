use core::fmt;
use std::ops::{Add, Sub, Mul};

use super::{util::almost_equal, tuple::Tuple};

use std::f64::consts::PI;
fn add_matrix (matrix1: &Matrix, matrix2: &Matrix, sign: bool) -> Matrix {
    let mut rows = Vec::new();
    let sign = if sign { 1.0 } else { -1.0 };
    for i in 0..matrix1.rows.len() {
        let mut row = Vec::new();
        for j in 0..matrix1.rows[i].len() {
            row.push(matrix1.rows[i][j] + sign * matrix2.rows[i][j]);
        }
        rows.push(row);
    }
    Matrix::new(rows)
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: Vec<Vec<f64>>,
    pub columns: Vec<Vec<f64>>,
}


impl Matrix {
    pub fn new(rows: Vec<Vec<f64>>) -> Self {
        if rows.is_empty() {
            return Self { rows, columns: Vec::new() };
        }
        let mut columns = Vec::new();
        for i in 0..rows[0].len() {
            let mut column = Vec::new();
            for row in rows.iter() {
                column.push(row[i]);
            }
            columns.push(column);
        }
        Self { rows, columns }
    }

    pub fn identity(size: u32) -> Self {
        let mut rows = Vec::new();
        for i in 0..size {
            let mut row = Vec::new();
            for j in 0..size {
                if i == j {
                    row.push(1.0);
                } else {
                    row.push(0.0);
                }
            }
            rows.push(row);
        }
        Matrix::new(rows)
    }

    pub fn transpose(&self) -> Self {
        Matrix::new(self.columns.clone())
    }

    pub fn determinant(&self) -> f64 {
        if self.rows.len() != self.rows[0].len() {
            panic!("Matrix must be square");
        } else
        if self.rows.len() == 1 {
            self.rows[0][0]
        } else 
        if self.rows.len() == 2 {
            self.rows[0][0] * self.rows[1][1] - self.rows[0][1] * self.rows[1][0]
        } else {
            let mut sum = 0.0;
            for i in 0..self.rows[0].len() {
                sum += self.rows[0][i] * self.cofactor(0, i);
            }
            sum
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Self {
        let mut rows = Vec::new();
        for i in 0..self.rows.len() {
            if i == row {
                continue;
            }
            let mut row = Vec::new();
            for j in 0..self.rows[i].len() {
                if j == column {
                    continue;
                }
                row.push(self.rows[i][j]);
            }
            rows.push(row);
        }
        Matrix::new(rows)
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            let inv_det = 1.0 / det;
            let mut rows = Vec::new();
            for i in 0..self.rows.len() {
                let mut row = Vec::new();
                for j in 0..self.rows[i].len() {
                    row.push(self.cofactor(j, i) * inv_det);
                }
                rows.push(row);
            }
            Some(Matrix::new(rows))
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Matrix::new(vec![
            vec![1.0, 0.0, 0.0, x],
            vec![0.0, 1.0, 0.0, y],
            vec![0.0, 0.0, 1.0, z],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Matrix::new(vec![
            vec![x, 0.0, 0.0, 0.0],
            vec![0.0, y, 0.0, 0.0],
            vec![0.0, 0.0, z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_x(radians: f64) -> Self {
        Matrix::new(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, radians.cos(), -radians.sin(), 0.0],
            vec![0.0, radians.sin(), radians.cos(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(radians: f64) -> Self {
        Matrix::new(vec![
            vec![radians.cos(), 0.0, radians.sin(), 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![-radians.sin(), 0.0, radians.cos(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    
    }

    pub fn rotation_z(radians: f64) -> Self {
        Matrix::new(vec![
            vec![radians.cos(), -radians.sin(), 0.0, 0.0],
            vec![radians.sin(), radians.cos(), 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::new(vec![
            vec![1.0, xy, xz, 0.0],
            vec![yx, 1.0, yz, 0.0],
            vec![zx, zy, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        add_matrix(&self, &other, true)
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        add_matrix(&self, &other, false)
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut rows = Vec::new();
        for i in 0..self.rows.len() {
            let mut row = Vec::new();
            for j in 0..other.columns.len() {
                let mut sum = 0.0;
                for k in 0..self.rows[i].len() {
                    sum += self.rows[i][k] * other.columns[j][k];
                }
                row.push(sum);
            }
            rows.push(row);
        }
        Matrix::new(rows)
    }
}


impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                if !almost_equal(self.rows[i][j], other.rows[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let x = other.x * self.rows[0][0] + other.y * self.rows[0][1] + other.z * self.rows[0][2] + other.w * self.rows[0][3];
        let y = other.x * self.rows[1][0] + other.y * self.rows[1][1] + other.z * self.rows[1][2] + other.w * self.rows[1][3];
        let z = other.x * self.rows[2][0] + other.y * self.rows[2][1] + other.z * self.rows[2][2] + other.w * self.rows[2][3];
        let w = other.x * self.rows[3][0] + other.y * self.rows[3][1] + other.z * self.rows[3][2] + other.w * self.rows[3][3];
        Tuple::new(x, y, z, w)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for val in row {
                write!(f, "{:.3}\t", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matrix_addition() {
        let matrix1 = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let matrix2 = Matrix::new(vec![vec![7.0, 8.0, 9.0], vec![10.0, 11.0, 12.0]]);
        let matrix3 = Matrix::new(vec![vec![8.0, 10.0, 12.0], vec![14.0, 16.0, 18.0]]);
        assert_eq!(matrix1 + matrix2, matrix3);
    }

    #[test]
    fn matrix_subtraction() {
        let matrix1 = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let matrix2 = Matrix::new(vec![vec![7.0, 8.0, 9.0], vec![10.0, 11.0, 12.0]]);
        let matrix3 = Matrix::new(vec![vec![-6.0, -6.0, -6.0], vec![-6.0, -6.0, -6.0]]);
        assert_eq!(matrix1 - matrix2, matrix3);
    }

    #[test]
    fn identity_matrix() {
        let matrix1 = Matrix::identity(4);
        let matrix2 = Matrix::new(vec![vec![1.0, 0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0, 0.0], vec![0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 0.0, 1.0]]);
        assert_eq!(matrix1, matrix2);
    }
    #[test]
    fn transpose_matrix() {
        let matrix1 = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let matrix2 = Matrix::new(vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);
        assert_eq!(matrix1.transpose(), matrix2);
    }
    
    #[test]
    fn determinant_matrix() {
        let matrix1 = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]]);
        assert_eq!(matrix1.determinant(), 0.0);
    }

    #[test]
    fn submatrix_matrix() {
        let matrix1 = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]]);
        let matrix2 = Matrix::new(vec![vec![1.0, 3.0], vec![7.0, 9.0]]);
        assert_eq!(matrix1.submatrix(1, 1), matrix2);
    }

    #[test]
    fn minor_matrix() {
        let matrix1 = Matrix::new(vec![vec![3.0, 5.0, 0.0], vec![2.0, -1.0, -7.0], vec![6.0, -1.0, 5.0]]);
        let matrix2 = matrix1.submatrix(1, 0);
        assert_eq!(matrix2.determinant(), 25.0);
        assert_eq!(matrix1.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_matrix() {
        let matrix1 = Matrix::new(vec![vec![3.0, 5.0, 0.0], vec![2.0, -1.0, -7.0], vec![6.0, -1.0, 5.0]]);
        assert_eq!(matrix1.cofactor(0, 0), -12.0);
    }

    #[test]
    fn cofactor_matrix_2() {
        let matrix1 = Matrix::new(vec![vec![3.0, 5.0, 0.0], vec![2.0, -1.0, -7.0], vec![6.0, -1.0, 5.0]]);
        assert_eq!(matrix1.cofactor(1, 0), -25.0);
    }

    #[test]
    fn inverse_matrix() {
        let a = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let expected = Matrix::new(vec![vec![-2.0, 1.0], vec![1.5, -0.5]]);
        assert_eq!(a.inverse(), Some(expected));
    
        let b = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]]);
        assert_eq!(b.inverse(), None);
    }

    #[test]
    fn translation_matrix() {
        let point = Tuple::point(1.0, 2.0, 3.0);
        let translation = Matrix::translation(2.0, 3.0, 4.0);
        let expected = Tuple::point(3.0, 5.0, 7.0);
        let transformed_point = translation * point; 
        assert_eq!(expected, transformed_point);
    }

    #[test]
    fn vector_translation() {
        let vector = Tuple::vector(1.0, 2.0, 3.0);
        let translation = Matrix::translation(2.0, 3.0, 4.0);
        let transformed_vector = translation * vector; 
        assert_eq!(vector, transformed_vector);
    }

    #[test]
    fn point_scaling() {
        let point = Tuple::point(-4.0,6.0, 8.0);
        let scaling_matrix = Matrix::scaling(2.0, 3.0, 4.0);
        let expected = Tuple::point(-8.0, 18.0, 32.0);
        assert_eq!(scaling_matrix * point, expected);
    }

    #[test]
    fn vector_scaling() {
        let vector = Tuple::vector(1.0,2.0, 3.0);
        let scaling_matrix = Matrix::scaling(2.0, 3.0, 4.0);
        let expected = Tuple::vector(2.0, 6.0, 12.0);
        assert_eq!(scaling_matrix * vector, expected);
    }

    #[test]
    fn vector_inverse_scaling() {
        let vector = Tuple::vector(-4.0,6.0, 8.0);
        let scaling_matrix = Matrix::scaling(2.0, 3.0, 4.0);
        let inverted_matrix = scaling_matrix.inverse().unwrap();
        let expected = Tuple::vector(-2.0, 2.0, 2.0);
        assert_eq!(inverted_matrix * vector, expected);
    }

    #[test]
    fn reflection() {
        let point = Tuple::point(1.0, 2.0, 3.0);
        let scaling_matrix = Matrix::scaling(-1.0, 1.0, 1.0);
        let expected = Tuple::point(-1.0, 2.0, 3.0);
        assert_eq!(scaling_matrix * point, expected);
    }

    #[test]
    fn rotation_x_point() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI/4.0);
        let full_quarter = Matrix::rotation_x(PI/2.0);
        let expected_half_quarter = Tuple::point(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0);
        let expected_full_quarter = Tuple::point(0.0, 0.0, 1.0);
        assert_eq!(half_quarter * point, expected_half_quarter);
        assert_eq!(full_quarter * point, expected_full_quarter);
    }

    #[test]
    fn rotation_x_inverse() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI/4.0);
        let inverted_matrix = half_quarter.inverse().unwrap();
        let expected = Tuple::point(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
        assert_eq!(inverted_matrix * point, expected);
    }

    #[test]
    fn rotation_y_point() {
        let point = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI/4.0);
        let full_quarter = Matrix::rotation_y(PI/2.0);
        let expected_half_quarter = Tuple::point(2.0_f64.sqrt()/2.0, 0.0, 2.0_f64.sqrt()/2.0);
        let expected_full_quarter = Tuple::point(1.0, 0.0, 0.0);
        assert_eq!(half_quarter * point, expected_half_quarter);
        assert_eq!(full_quarter * point, expected_full_quarter);
    }

    #[test]
    fn rotation_z_point() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI/4.0);
        let full_quarter = Matrix::rotation_z(PI/2.0);
        let expected_half_quarter = Tuple::point(-2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 0.0);
        let expected_full_quarter = Tuple::point(-1.0, 0.0, 0.0);
        assert_eq!(half_quarter * point, expected_half_quarter);
        assert_eq!(full_quarter * point, expected_full_quarter);
    }

    #[test]
    fn shearing_x_in_proportion_to_y() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let shear_matrix = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let expected = Tuple::point(5.0, 3.0, 4.0);
        assert_eq!(shear_matrix * point, expected);
    }

    #[test]
    fn individual_transformations_applied_in_sequence() {
        let point = Tuple::point(1.0, 0.0, 1.0);
        let rotation = Matrix::rotation_x(PI / 2.0);
        let p2 = rotation * point;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let p3 = scaling * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));
        let translation = Matrix::translation(10.0, 5.0, 7.0);
        let p4 = translation * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_applied_in_reverse_order() {
        let point = Tuple::point(1.0, 0.0, 1.0);
        let rotation = Matrix::rotation_x(PI / 2.0);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);
        let transform = translation * scaling * rotation;
        assert_eq!(transform * point, Tuple::point(15.0, 0.0, 7.0));
    }
}
