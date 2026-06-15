use std::ops::{Add, Mul, Sub};
/// A single element of the finite field F_16
/// 
/// Only the lower 4 bits are used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F16Element(pub u8);

#[cfg(test)]
#[path = "tests/fieldTests.rs"]
mod tests;

/// Look-up table for multiplication 
const MUL_TABLE: [[u8; 16]; 16] = build_mul_table();
/// Look-up table for inversion
const INV_TABLE: [u8; 16] = build_inv_table();

/// Addition and subtraction is XOR
impl Add for F16Element {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        F16Element(self.0 ^ other.0)
    }
}
impl Sub for F16Element {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        F16Element(self.0 ^ other.0)
    }
}

/// Multiplication is polynomial multiplication modulo x⁴+x+1
impl Mul for F16Element {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        F16Element(MUL_TABLE[self.0 as usize][other.0 as usize])
    }
}

impl F16Element {

    /// The multiplicative inverse - used in Gaussian elimination
    fn inverse(self) -> Self{
        assert!(self.0 != 0, "zero has no inverse");
        F16Element(INV_TABLE[self.0 as usize])
    }

    pub fn new(val: u8) -> Self {
        F16Element(val & 0xF)
    }
    
    pub fn zero() -> Self {
        F16Element(0)
    }
}

/// Precompute a 16x16 table for fast Look-up 
const fn build_mul_table() -> [[u8; 16]; 16]{

    let mut table = [[0u8; 16]; 16];

    let mut a = 0;
    while a < 16 {

        let mut b = 0;
        while b < 16 {
            
            table[a][b] = gf16_mul(a as u8, b as u8);
            b += 1;
        }
        a += 1;
    }

    table
}
/// Precompute a table for inversion
const fn build_inv_table() -> [u8; 16] {
    let mut table = [0u8; 16];
    let mut a = 1; // skip 0, has no inverse
    while a < 16 {
        table[a] = gf16_inv(a as u8);
        a += 1;
    }

    table
}

/// borrowed from https://github.com/pqov/pqov-paper/blob/main/src/gf16.h 
const fn gf16_mul(a: u8, b: u8) -> u8 {
    let mut r8: u8 = (a&1)*b;
    r8 ^= (a&2)*b;
    r8 ^= (a&4)*b;
    r8 ^= (a&8)*b;

    // reduction
    let mut r4: u8 = r8 ^ (((r8>>4)&5)*3); // x^4 = x+1  , x^6 = x^3 + x^2
    r4 ^= ((r8>>5)&1)*6;             // x^5 = x^2 + x
    
    return r4&0xf;
}

/// borrowed from https://github.com/pqov/pqov-paper/blob/main/src/gf16.h 
const fn gf16_squ(a: u8) -> u8{
    let mut r4 = a&1;    // constant term
    r4 ^= (a<<1)&4;      // x -> x^2
    r4 ^= ((a>>2)&1)*3;  // x^2 -> x^4 -> x+1
    r4 ^= ((a>>3)&1)*12; // x^3 -> x^6 -> x^3+x^2
    
    return r4;
}

/// borrowed from https://github.com/pqov/pqov-paper/blob/main/src/gf16.h 
const fn gf16_inv(a: u8) -> u8{
    // fermat inversion
    let a2: u8 = gf16_squ(a);
    let a4: u8 = gf16_squ(a2);
    let a8: u8 = gf16_squ(a4);
    let a6: u8 = gf16_mul(a4, a2);
    
    return gf16_mul(a8, a6);
}