use crate::matrix::FieldMatrix;

/// Contains the m matrices P³, each of shape m × m
/// 
/// P¹ and P² are not stored - they can be regenerated from seed_pk
pub struct PublicKey {
    p3_matrices: Vec<FieldMatrix>
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

/// Generate a fresh key pair
/// 
/// Randomly samples seed_sk and seed_pk, derives everything else
fn keygen() -> (PublicKey, SecretKey){
    todo!()
}