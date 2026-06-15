
use super::*;

#[test]
fn test_hash() {
    let message = b"Unbalanced Oil and Vinegar";
    let salt = b"1234567890123456";
    let m = 64;

    let hashed = hash(message, salt, m);
    assert_eq!(hashed.0.len(), m, "The Target vector must have exactly M elements");
    for elem in hashed.0.iter() {
            assert!(elem.0 < 16, "Element out of bounds for F_16: {}", elem.0);
    }
}

#[test]
fn test_expandsk() {
    let seed_sk = [0u8; 32];
    let o_matrix = expand_sk(&seed_sk);
    let expected_v = 96;
    let expected_m = 64;

    assert_eq!(o_matrix.rows(), expected_v);
    assert_eq!(o_matrix.cols(), expected_m);
    assert_eq!(o_matrix.data().len(), expected_v * expected_m);
}

#[test]
fn test_expandp() {
    let seed_pk = [0u8; 16];
    let expected_v = 96;
    let expected_m = 64;

    let (p1_matrices, p2_matrices) = expand_p(&seed_pk);

    assert_eq!(p1_matrices.len(), expected_m, "Should generate exactly M P1 matrices");
    assert_eq!(p2_matrices.len(), expected_m, "Should generate exactly M P2 matrices");

    // V x V and Upper Triangular
    for p1 in p1_matrices.iter() {
        assert_eq!(p1.rows(), expected_v);
        assert_eq!(p1.cols(), expected_v);

        for i in 0..expected_v {
            for j in 0..expected_v {
                // In an upper triangular matrix, any element where row > col must be 0
                if i > j {
                    let elem = p1.get(i, j); 
                    assert_eq!(elem.0, 0, "Lower triangle of P1 must be zero at ({}, {})", i, j);
                }
            }
        }
    }
    // V x M 
    for p2 in p2_matrices.iter() {
        assert_eq!(p2.rows(), expected_v);
        assert_eq!(p2.cols(), expected_m);
    }
}

#[test]
fn test_expandv() {
    let message = b"Unbalanced Oil and Vinegar";
    let salt = b"1234567890123456";
    let seed = [1u8; 32];

    let v1 = expand_v(message, salt, &seed, 0);
    let v2 = expand_v(message, salt, &seed, 0);
    assert_eq!(v1.0, v2.0, "Hash must be deterministic for the same counter");

    let v3 = expand_v(message, salt, &seed, 1);
    assert_ne!(v1.0, v3.0, "Increasing the counter must produce a new vinegar vector");
}