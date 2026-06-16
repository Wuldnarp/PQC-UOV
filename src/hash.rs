use crate::field::F16Element;
use crate::matrix::FieldMatrix;
use crate::matrix::FieldVector;
use ctr::cipher::KeyIvInit;
use ctr::cipher::StreamCipher;
use shake::Shake256;
use shake::XofReader;
use shake::digest::{Update, ExtendableOutput};

pub type Aes128Ctr = ctr::Ctr128BE<aes::Aes128>;

#[cfg(test)]
#[path = "tests/hashTests.rs"]
mod tests;

// Parameters for ov-Is
// Maybe not the best location if others need to use and stuff but yes
const M: usize = 64; 
const V: usize = 96;

// Helper function for unpacking bytes into a vector of F16 elements
// Extracts the least significant nibble first and the most significant nibble.
fn unpack_f16(buffer: &[u8], count: usize) -> Vec<F16Element> {
    let mut elements = Vec::with_capacity(count);
    
    for &byte in buffer {
        elements.push(F16Element::new(byte & 0x0F));
        if elements.len() == count {
            break;
        }
        
        elements.push(F16Element::new((byte >> 4) & 0x0F));
        if elements.len() == count {
            break;
        }
    }
    elements
}

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
    // Maybe not the right way to do it with code
    // squeezing the SHAKE256 output and unpacking the bytes into F16 elements.
    // I will look into it
    let bytes_to_read = (m+1)/2;
    let mut buffer = vec![0u8; bytes_to_read];
    reader.read(&mut buffer);

    FieldVector::new(unpack_f16(&buffer, m))
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
    
    let total_elements = V * M;
    let bytes_to_read = (total_elements+1)/2;
    let mut buffer = vec![0u8; bytes_to_read];
    reader.read(&mut buffer);

    FieldMatrix::new(V, M, unpack_f16(&buffer, total_elements))
}

fn expand_p(seed_pk: &[u8]) -> (Vec<FieldMatrix>, Vec<FieldMatrix>) {
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

    let mut elements = Vec::with_capacity(total_elements);
    for byte in keystream {
        elements.push(F16Element::new(byte & 0x0F));
        if elements.len() == total_elements {
            break;
        }
        elements.push(F16Element::new((byte >> 4) & 0x0F));
        if elements.len() == total_elements {
            break;
        }
    }

    let mut element_iter = elements.into_iter();
    let mut p1_data_blocks = vec![vec![F16Element::new(0); V * V]; M];
    let mut p2_data_blocks = vec![vec![F16Element::new(0); V * M]; M];
    for i in 0..V {
        for j in i..V {
            for k in 0..M {
                if let Some(elem) = element_iter.next() {
                    p1_data_blocks[k][i * V + j] = elem;
                }
            }
        }
    }
 
    for i in 0..V {
        for j in 0..M {
            for k in 0..M {
                if let Some(elem) = element_iter.next() {
                    p2_data_blocks[k][i * M + j] = elem;
                }
            }
        }
    }

    let p1_matrices = p1_data_blocks
        .into_iter()
        .map(|data| FieldMatrix::new(V, V, data))
        .collect();

    let p2_matrices = p2_data_blocks
        .into_iter()
        .map(|data| FieldMatrix::new(V, M, data)) 
        .collect();

    (p1_matrices, p2_matrices)
}

/// Generate the vinegar vector v ∈ F_16^(n-m) for a signing attempt
/// 
/// ctr needs to increment each time Gaussian elimination fails, to give a fresh v to retry with
/// 
/// Implemented as SHAKE256(message || salt || seed_sk || ctr)
fn expand_v(message: &[u8], salt: &[u8], seed: &[u8], ctr: u8) -> FieldVector{
        const V: usize = 96;
    let mut hasher = Shake256::default();
    hasher.update(message);
    hasher.update(salt);
    hasher.update(seed);
    hasher.update(&[ctr]);
    let mut reader = hasher.finalize_xof();
    
    let bytes_to_read = (V + 1) / 2;
    let mut buffer = vec![0u8; bytes_to_read];
    
    reader.read(&mut buffer);

    FieldVector::new(unpack_f16(&buffer, V))
}