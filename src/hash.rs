use crate::matrix::FieldMatrix;
use crate::matrix::FieldVector;

/// Hash the message with salt to produce the target vector t ∈ F_16^m
/// 
/// t is what the signature must map to under the public polynomials
/// 
/// Implemented as SHAKE256(message || salt)
/// 
/// output is m F_16 elements
fn hash(message: &[u8], salt: &[u8], m: usize) -> FieldVector{
    todo!()
}

/// Expand the secret seed into the trapdoor matrix O of shape (n-m) × m
/// 
/// O is the oil subspace - the secret that makes signing possible
/// 
/// Implemented as SHAKE256(seed_sk)
/// 
/// output is (n-m)*m F_16 elements
fn expand_sk(seed: &[u8]) -> FieldMatrix{
    todo!()
}

/// Expand the public seed into the random matrices P¹ and P²
/// 
/// P¹ has shape (n-m) × (n-m), P² has shape (n-m) × m
/// 
/// Implemented as AES128-CTR(seed_pk)
fn expand_pk(seed: &[u8]) -> (Vec<FieldMatrix>, Vec<FieldMatrix>){
    todo!()
}

/// Generate the vinegar vector v ∈ F_16^(n-m) for a signing attempt
/// 
/// ctr needs to increment each time Gaussian elimination fails, to give a fresh v to retry with
/// 
/// Implemented as SHAKE256(message || salt || seed_sk || ctr)
fn expand_v(message: &[u8], salt: &[u8], seed: &[u8], ctr: u8) -> FieldVector{
    todo!()
}