use crate::field::F16Element;
use std::ops::{Add, Mul};
/// A column vector of F_16 elements
#[derive(Debug, PartialEq, Clone)]
pub struct FieldVector(pub Vec<F16Element>);

/// A matrix of F_16 elements
#[derive(Debug, PartialEq, Clone)]
pub struct FieldMatrix {
    rows: usize,
    cols: usize,
    data: Vec<F16Element>
}

#[cfg(test)]
#[path = "tests/matrixTests.rs"]
mod tests;

/// addition of two vectors (XOR in F_16)
impl Add for FieldVector{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.0.len(), other.0.len());
        FieldVector(self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a,b)| *a + *b)
            .collect()
        )
    }
}

impl FieldVector{
    pub fn new(data: Vec<F16Element>) -> Self {
        FieldVector(data)
    }
}

/// add for matrix 
impl Add for FieldMatrix {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let data = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| *a + *b)
            .collect();
        FieldMatrix { rows: self.rows, cols: self.cols, data }
    }
}

/// Multiply with matrix
impl Mul for FieldMatrix {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.multiply_with_matrix(other)
    }
}

/// Multiply with column vector
impl Mul<FieldVector> for FieldMatrix{
    type Output = FieldVector;
    fn mul(self, v: FieldVector) -> Self::Output {
        self.multiply_with_vector(v)
    }
}

impl FieldMatrix {

    pub fn new(rows: usize, cols: usize, data: Vec<F16Element>) -> Self {
        assert_eq!(data.len(), rows * cols, "data length must equal rows * cols");
        FieldMatrix { rows, cols, data }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &[F16Element] {
        &self.data
    }

    pub fn get(&self, row: usize, col: usize) -> F16Element {
        self.data[row * self.cols + col]
    }

    fn get_row(&self, row: usize) -> &[F16Element] {
        &self.data[row * self.cols .. (row + 1) * self.cols]
    }

    fn set(&mut self, row: usize, col: usize, val: F16Element) {
        self.data[row * self.cols + col] = val;
    }

    /// Multiply matrix with another matrix
    pub fn multiply_with_matrix(&self, other: Self) -> Self {
        assert_eq!(self.cols, other.rows);

        let mut data = Vec::with_capacity(self.rows * other.cols);

        for i in 0..self.rows {
            let row = self.get_row(i);
            for k in 0..other.cols {
                let sum = row.iter()
                    .enumerate()
                    .map(|(j, &a_ij)| a_ij * other.get(j, k))
                    .fold(F16Element::zero(), |acc, x| acc + x);
                data.push(sum);
            }
        }

        FieldMatrix { rows: self.rows, cols: other.cols, data }
    }

    /// Multiply matrix by column vector
    /// 
    /// Produces column vector of length rows
    pub fn multiply_with_vector(&self, v: FieldVector) -> FieldVector{
        assert_eq!(self.cols, v.0.len());

        let result = (0..self.rows)
        .map(|i| {
            let row = self.get_row(i);
            row.iter()
                .zip(v.0.iter())
                .map(|(a, b)| *a * *b)
                .fold(F16Element::zero(), |acc, x| acc + x)
        })
        .collect();

        FieldVector(result)
    }

    /// Transpose the matrix (swap rows and cols)
    pub fn transpose(&self) -> Self{
        let mut data = Vec::with_capacity(self.rows * self.cols);

        for j in 0..self.cols {
            for i in 0..self.rows {
                data.push(self.get(i, j));
            }
        }

        FieldMatrix {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }

    /// Extract the upper triangular part of a matrix
    pub fn upper(&self) -> Self{
        assert_eq!(self.rows, self.cols);

        let n = self.rows;
        let mut data = vec![F16Element::zero(); n * n];

        for idx in 0..self.data.len() {
            let i = idx / n;
            let j = idx % n;
            if i <= j { // upper triangle + diagonal
                data[idx] =
                if i == j { // diagonal
                    self.data[idx]
                } else { // upper triangle
                    self.data[idx] + self.data[j * n + i]
                };
            }
        }

        FieldMatrix { rows: n, cols: n, data }
    }

    /// Solve the linear system using Gaussian elimination
    /// 
    /// returns None if if the system has no unique solution (matrix not invertible)
    pub fn gaussian_elimination(&self, rhs: &FieldVector) -> Option<FieldVector>{
        assert_eq!(self.rows, self.cols);
        let m = self.rows;
        assert_eq!(m, rhs.0.len());

        let width = m + 1;

        // Build augmented matrix L' = (L | rhs), size m × (m+1)
        let mut l = FieldMatrix::new(m, m + 1, vec![F16Element::zero(); m * (m + 1)]);

        for i in 0..m {
            for j in 0..m {
                l.set(i, j, self.get(i, j));
            }
            l.set(i, m, rhs.0[i]);
        }

        // Forward elimination (lines 2-14)
        for i in 0..m {
            // Lines 3-6: conditionally add later rows to fix zero pivot
            for j in (i + 1)..m {
                if l.get(i, i) == F16Element::zero() {
                    for k in i..width {
                        l.set(i, k, l.get(i, k) + l.get(j, k));
                    }
                }
            }
            // Line 7: early abort
            if l.get(i, i) == F16Element::zero() {
                return None; // singular
            }

            // Lines 8-10: normalize pivot row
            let p_inv = l.get(i, i).inverse();
            for k in i..width {
                l.set(i, k, l.get(i, k) * p_inv);
            }

            // Lines 11-14: eliminate column i from all other rows
            for j in 0..m {
                if j != i {
                    let factor = l.get(j, i);
                    if factor != F16Element::zero() {
                        for k in i..width {
                            l.set(j, k, l.get(j, k) + factor * l.get(i, k));
                        }
                    }
                }
            }
        }

        // Extract last column as the solution
        let result = (0..m).map(|i| l.get(i, m)).collect();
        Some(FieldVector(result))
    }
}