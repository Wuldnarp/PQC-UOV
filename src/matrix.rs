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

/// Multiply with column vector
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

    fn get_row(&self, i: usize) -> &[F16Element] {
        &self.data[i * self.cols .. (i + 1) * self.cols]
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
        todo!()
    }


    /// Negate the matrix
    pub fn negate(&self) -> Self{
        let mut data = Vec::with_capacity(self.rows * self.cols);

        for j in 0..self.rows * self.cols {
            for i in 0..self.rows {
                data.push(-self.get(i, j));
            }
        }

        FieldMatrix {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }

    /// Solve the linear system using Gaussian elimination
    /// 
    /// returns None if if the system has no unique solution (matrix not invertible)
    pub fn gaussian_elimination(&self, rhs: &FieldVector) -> Option<FieldVector>{
        todo!()
    }
}