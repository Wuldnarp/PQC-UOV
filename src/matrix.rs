use crate::field::F16Element;
/// A column vector of F_16 elements 
pub struct FieldVector(Vec<F16Element>);

/// A matrix of F_16 elements
pub struct FieldMatrix {
    rows: usize,
    cols: usize,
    data: Vec<F16Element>
}

impl FieldVector {

    /// addition of two vectors (XOR in F_16)
    fn add(&self, other: &Self) -> Self{
        todo!()
    }
}

impl FieldMatrix {

    /// Multiply matrix (rows × cols) by column vector of length cols
    /// 
    /// Produces column vector of length rows
    fn multiply_with_vector(&self, v: &FieldVector) -> FieldVector{
        todo!()
    }

    /// Multiply two matrices together
    fn multiply_with_matrix(&self, other: &Self) -> Self{
        todo!()
    }

    /// Transpose the matrix (swap rows and cols)
    fn transpose(&self) -> Self{
        todo!()
    }

    /// Extract the upper triangular part of a matrix
    fn upper(&self) -> Self{
        todo!()
    }

    /// Solve the linear system using Gaussian elimination
    /// 
    /// returns None if if the system has no unique solution (matrix not invertible)
    fn gaussian_elimination(&self, rhs: &FieldVector) -> Option<FieldVector>{
        todo!()
    }
}