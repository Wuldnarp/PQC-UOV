use super::*;

#[test]
fn test_add_and_sub(){

    let a = F16Element(0b0110);
    let b = F16Element(0b0011);
    
    assert_eq!((a+b).0, 0b0101);
}
#[test]
fn test_mul_by_zero() {

    let a = F16Element(0b1101);
    let zero = F16Element(0);

    assert_eq!((a * zero).0, 0);
}

#[test]
fn test_mul_by_one() {

    let a = F16Element(0b1101);
    let one = F16Element(1);

    assert_eq!((a * one).0, a.0);
}

#[test]
fn test_inverse() {

    for x in 1..16u8 {

        let a = F16Element(x);
        let inv = a.inverse();

        assert_eq!((a * inv).0, 1, "a={} * inv={} should be 1", a.0, inv.0);
    }
}

#[test]
fn test_add_self_inverse() {

    let a = F16Element(0b1010);

    assert_eq!((a + a).0, 0);
}

#[test]
#[should_panic(expected = "zero has no inverse")]
fn test_zero_has_no_inverse() {

    let zero = F16Element(0);
    let _ = zero.inverse();
}