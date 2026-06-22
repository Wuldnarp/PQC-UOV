use crate::matrix::FieldMatrix;
use crate::hash::{expand_sk, expand_p};
use rand::rngs::OsRng;
use rand::TryRngCore;


/// Contains the m matrices P³, each of shape m × m
/// 
/// P¹ and P² are not stored - they can be regenerated from seed_pk
pub struct PublicKey {
    pub p3_matrices: Vec<FieldMatrix>,
    pub seed_pk: [u8; 16],
}
impl PublicKey {
    pub fn new(matrices: Vec<FieldMatrix>, seed_pk: [u8; 16]) -> PublicKey {
        PublicKey{p3_matrices: matrices, seed_pk}
    }
}

/// Contains everything needed to sign a message
pub struct SecretKey {

    /// Seed for generating the trapdoor O via expand_sk
    pub seed_sk: [u8; 32],

    /// The trapdoor matrix O of shape (n-m) × m
    pub o_matrix: FieldMatrix,

    /// The m matrices P¹, each of shape (n-m) × (n-m)
    /// 
    /// Stored here for fast signing
    pub p1_matrices: Vec<FieldMatrix>,

    /// The precomputed matrices S, one per polynomial
    /// 
    /// Stored to avoid recomputing during every signing operation
    pub s_matrices: Vec<FieldMatrix>
}

impl SecretKey {
    pub fn new(
        seed_sk: [u8; 32],
        o_matrix: FieldMatrix,
        p1_matrices: Vec<FieldMatrix>,
        s_matrices: Vec<FieldMatrix>) -> SecretKey
    {
        Self{
            seed_sk,
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

    // O: V × M (secret oil subspace)
    let o = expand_sk(&seed_sk);
    let ot = o.transpose();

    let (p1,p2) = expand_p(&seed_pk);
    let m = p1.len();

    // compute p3 and s matrices
    let mut p3: Vec<FieldMatrix> = Vec::with_capacity(m);
    let mut s_matrices: Vec<FieldMatrix> = Vec::with_capacity(m);

    for i in 0..m {
        // P3_i = Upper(O^T P1_i O + O^T P2_i)
        let ot_p1_o = ot.clone() * p1[i].clone() * o.clone();
        let ot_p2 = ot.clone() * p2[i].clone();

        p3.push((ot_p1_o + ot_p2).upper());

        // S_i = (P1_i + P1_i^T) * O + P2_i
        let p1_sym = p1[i].clone() + p1[i].transpose();
        let s = p1_sym * o.clone() + p2[i].clone();
        s_matrices.push(s);
    }

    let sk = SecretKey::new(seed_sk, o, p1, s_matrices);
    let pk = PublicKey::new(p3,seed_pk);

    (pk, sk)
}