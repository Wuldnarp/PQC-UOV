use crate::matrix::FieldMatrix;
use crate::hash::{expand_sk, expand_p, expand_v};
use rand::rngs::OsRng;
use rand::TryRngCore;


/// Contains the m matrices P³, each of shape m × m
/// 
/// P¹ and P² are not stored - they can be regenerated from seed_pk
pub struct PublicKey {
    p3_matrices: Vec<FieldMatrix>
}
impl PublicKey {
    pub fn new(matrices: Vec<FieldMatrix>) -> PublicKey {
        PublicKey{p3_matrices: matrices}
    }
}

/// Contains everything needed to sign a message
pub struct SecretKey {

    /// Seed for generating the trapdoor O via expand_sk
    seed_sk: [u8; 32],

    /// Seed for regenerating P¹ and P² via expand_pk
    seed_pk: [u8; 16],

    /// The trapdoor matrix O of shape (n-m) × m
    o_matrix: FieldMatrix,

    /// The m matrices P¹, each of shape (n-m) × (n-m)
    /// 
    /// Stored here for fast signing
    p1_matrices: Vec<FieldMatrix>,

    /// The precomputed matrices S, one per polynomial
    /// 
    /// Stored to avoid recomputing during every signing operation
    s_matrices: Vec<FieldMatrix>
}

impl SecretKey {
    pub fn new(
        seed_sk: [u8; 32],
        seed_pk: [u8; 16],
        o_matrix: FieldMatrix,
        p1_matrices: Vec<FieldMatrix>,
        s_matrices: Vec<FieldMatrix>) -> SecretKey
    {
        Self{
            seed_sk,
            seed_pk,
            o_matrix,
            p1_matrices,
            s_matrices
        }
    }
}

/// Generate a fresh key pair
///
/// Randomly samples seed_sk and seed_pk, derives everything else
pub fn keygen() -> (PublicKey, SecretKey)
{
    let mut rng = OsRng;
    let mut seed_sk = [0u8; 32];
    let mut seed_pk = [0u8; 16];
    rng.try_fill_bytes(&mut seed_sk).expect("seed_sk RNG failure");
    rng.try_fill_bytes(&mut seed_pk).expect("seed_pk RNG failure");

    let o = expand_sk(&seed_sk);
    // Does -OT means negative matrix transposed?
    let ot = o.clone().transpose();

    let (p1,p2) = expand_p(&seed_pk);
    let m = p1.len();

    let mut p3: Vec<FieldMatrix> = Vec::new();
    for i in 0..m-1 {
        let otp = ot.multiply_with_matrix(p2[i].clone());
        let opo = ot.multiply_with_matrix(p1[i].multiply_with_matrix(o.clone()));
        p3.push(opo.multiply_with_matrix(otp).upper());
    }

    let sk = SecretKey::new(seed_sk, seed_pk, ot, p1, p3.clone());
    let pk = PublicKey::new(p3.clone());

    (pk, sk)
}