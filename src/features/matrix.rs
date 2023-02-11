use std::ops::{Add, Sub, Mul};

use super::util::almost_equal;

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
        if (self.rows.len() != self.rows[0].len()) {
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
        let matrix1 = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]]);
        assert_eq!(matrix1.minor(1, 1), 0.0);
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
}