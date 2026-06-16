use super::*;

fn ev(vals: &[u8]) -> Vec<F16Element> {
    vals.iter().map(|&v| F16Element::new(v)).collect()
}

#[test]
fn test_multiply_2x2_with_vector() {

    // A = [[   1,    x],
    //      [ x+1,   x²]]
    let a = FieldMatrix::new(2, 2, ev(&[
        0b0001, 0b0010,
        0b0011, 0b0100,
    ]));

    // v = [x²+1, x²+x]
    let v = FieldVector::new(ev(&[0b0101, 0b0110]));

    let expected = FieldVector::new(ev(&[0b1001, 0b0100]));

    assert_eq!(a.multiply_with_vector(v), expected);
}

#[test]
fn test_multiply_2x2_with_2x2() {

    // A = [[   1,    x],
    //      [ x+1,   x²]]
    let a = FieldMatrix::new(2, 2, ev(&[
        0b0001, 0b0010,
        0b0011, 0b0100,
    ]));
    
    // B = [[x²+1,  x²+x],
    //      [x²+x+1,  x³]]
    let b = FieldMatrix::new(2, 2, ev(&[
        0b0101, 0b0110,
        0b0111, 0b1000,
    ]));

    // C = [[x³+x+1,  x²+1],
    //      [     0,  x³+x²]]
    let expected = FieldMatrix::new(2, 2, ev(&[
        0b1011, 0b0101,
        0b0000, 0b1100,
    ]));

    assert_eq!(a.multiply_with_matrix(b), expected);
}

#[test]
fn test_transpose_2x3() {

    // A = [[   1,    x,  x+1],
    //      [  x², x²+1, x²+x]]
    let a = FieldMatrix::new(2, 3, ev(&[
        0b0001, 0b0010, 0b0011,
        0b0100, 0b0101, 0b0110,
    ]));

    // A^T = [[  1,   x²],
    //        [  x, x²+1],
    //        [x+1, x²+x]]
    let expected = FieldMatrix::new(3, 2, ev(&[
        0b0001, 0b0100,
        0b0010, 0b0101,
        0b0011, 0b0110,
    ]));

    assert_eq!(a.transpose(), expected);
}

#[test]
fn test_upper_3x3() {

    // M = [[    1,      x,    x+1],
    //      [   x²,   x²+1,  x²+x],
    //      [x²+x+1,   x³, x³+1  ]]
    let m = FieldMatrix::new(3, 3, ev(&[
        0b0001, 0b0010, 0b0011,
        0b0100, 0b0101, 0b0110,
        0b0111, 0b1000, 0b1001,
    ]));

    // Upper folds lower into upper via XOR (same polynomial + same poly = 0 on diagonal off-elements):
    let expected = FieldMatrix::new(3, 3, ev(&[
        0b0001, 0b0110, 0b0100,
        0b0000, 0b0101, 0b1110,
        0b0000, 0b0000, 0b1001,
    ]));

    assert_eq!(m.upper(), expected);
}

#[test]
fn test_gaussian_elimination_3x3() {

    // M = [[    1,    x,  x+1],
    //      [    0,    1,   x²],
    //      [ x²+1, x²+x,   0]]
    let matrix = FieldMatrix::new(3, 3, ev(&[
        0b0001, 0b0010, 0b0011,
        0b0000, 0b0001, 0b0100,
        0b0101, 0b0110, 0b0000,
    ]));

    // x = [x, x²+x+1, x³+1]
    let x = FieldVector::new(ev(&[0b0010, 0b0111, 0b1001]));

    // rhs = M * x 
    let rhs = matrix.multiply_with_vector(x.clone());

    let solved = matrix.gaussian_elimination(&rhs)
        .expect("matrix should be invertible");

    assert_eq!(solved, x);
}