/// A single element of the finite field F_16
/// 
/// Only the lower 4 bits are used    
pub struct F16Element(u8);

impl F16Element {

    /// Addition is XOR
    fn add(self, other: Self) -> Self{
        todo!()
    }

    /// Multiplication is polynomial multiplication modulo x⁴+x+1
    fn multiply(self, other: Self) -> Self{
        todo!()
    }

    /// The multiplicative inverse - used in Gaussian elimination
    fn inverse(self) -> Self{
        todo!()
    }
}