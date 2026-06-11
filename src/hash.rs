use crate::field::F16Element;
use crate::matrix::FieldMatrix;
use crate::matrix::FieldVector;
use ctr::cipher::KeyIvInit;
use ctr::cipher::StreamCipher;
use shake::Shake256;
use shake::XofReader;
use shake::digest::{Update, ExtendableOutput};

pub type Aes128Ctr = ctr::Ctr128BE<aes::Aes128>;

/// Hash the message with salt to produce the target vector t ∈ F_16^m
/// 
/// t is what the signature must map to under the public polynomials
/// 
/// Implemen_reader SHAKE256(message || salt)
/// 
/// output is m F_16 elements
/// 
fn hash(message: &[u8], salt: &[u8], m: usize) -> FieldVector{
    let mut hasher = Shake256::default();
    hasher.update(message);
    hasher.update(salt);

    let mut reader = hasher.finalize_xof();
    // let elements = need some kind of unpacking
    //  For F16 , we pack two field elements into one byte with the first element in the least
    // significant nibble.
    // FieldVector(elements)
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
    let mut hasher = Shake256::default();
    hasher.update(seed);
    let mut reader = hasher.finalize_xof();
    todo!()
}

fn expand_p(seed_pk: &[u8]) -> (Vec<FieldMatrix>, Vec<FieldMatrix>) {
    // Maybe a better place to have parameters
    const M: usize = 64;  
    // n - m
    const V: usize = 96;

    // maybe the math aint mathing but will look at it
    // m * ((n-m)(n-m+1))/2 and sub (n+m) for V
    let p1_elements_per_matrix = (V * (V + 1)) / 2;
    let total_p1_elements = p1_elements_per_matrix * M;
    // (n-m) * m
    let total_p2_elements = (V * M) * M;
    let total_elements = total_p1_elements + total_p2_elements;
    // For F16​, we pack two field elements into one byte with the first element in the least significant nibble.
    let total_bytes = (total_elements + 1) / 2;

    let key: [u8; 16] = seed_pk.try_into().expect("seed_pk must be exactly 16 bytes");
    let iv = [0u8; 16];
    // Doesn't seem like the crate offers round-reduction to 4
    let mut cipher = Aes128Ctr::new(&key.into(), &iv.into());
    let mut keystream = vec![0u8; total_bytes];
    cipher.apply_keystream(&mut keystream);

    //Need a way to unpack bytes into F16Element instances
    // And split segments to populate matrices

    todo!()
}

/// Generate the vinegar vector v ∈ F_16^(n-m) for a signing attempt
/// 
/// ctr needs to increment each time Gaussian elimination fails, to give a fresh v to retry with
/// 
/// Implemented as SHAKE256(message || salt || seed_sk || ctr)
fn expand_v(message: &[u8], salt: &[u8], seed: &[u8], ctr: u8) -> FieldVector{
    // maybe
    let mut hasher = Shake256::default();
    hasher.update(message);
    hasher.update(salt);
    hasher.update(seed);
    hasher.update(&[ctr]);
    let mut reader = hasher.finalize_xof();
    todo!()
}