use crate::keys::{PublicKey, SecretKey};
use crate::hash::{expand_p, expand_v, hash_message};
use crate::field::F16Element;
use crate::matrix::{FieldMatrix, FieldVector};
use rand::rngs::OsRng;
use rand::TryRngCore;

// ov-Is parameters
const M: usize = 64;
const V: usize = 96;
const N: usize = V + M;
/// A signature on a message
pub struct Signature {
    /// Random 16-byte salt, chosen fresh for each signature    
    salt: [u8; 16],

    /// The actual signature vector s of length n
    /// 
    /// s = (v + Ox) || x
    s: FieldVector
}

/// Sign a message using the secret key
pub fn sign(sk: &SecretKey, message: &[u8]) -> Option<Signature>{
    
    // Fresh random salt
    let mut salt = [0u8; 16];
    OsRng.try_fill_bytes(&mut salt).expect("RNG failure");

    // Compute t
    let t = hash_message(message, &salt, M);

    for ctr in 0u8..=255 {

        // Expand v
        let v = expand_v(message, &salt, &sk.seed_sk, ctr);

        // Build L
        let mut l_data = Vec::with_capacity(M * M);
        
        for s_i in &sk.s_matrices {
            // row i of L = s_i^T * v
            let row = s_i.transpose() * v.clone();
            l_data.extend_from_slice(&row.0);
        } 
        let l = FieldMatrix::new(M, M, l_data);

        // y_i = v^T * P1_i * v 
        let y: FieldVector = FieldVector(
            sk.p1_matrices.iter().map(|p1_i| {
                // p1_i * v, then v^T * result
                let p1v = p1_i.clone() * v.clone();

                v.0.iter().zip(p1v.0.iter())
                    .map(|(vi, p1vi)| *vi * *p1vi)
                    .fold(F16Element::zero(), |acc, x| acc + x)
            }).collect()
        );

        // rhs = t - y (- is the same as + in F16)
        let rhs = t.clone() + y;

        // Solve Lx = rhs
        if let Some(x) = l.gaussian_elimination(&rhs) {
            
            // s = (v + O*x) || x
            let ox = sk.o_matrix.clone() * x.clone();
            let vox = v + ox;

            // Concatenate: s = v_part || x_part,
            let mut s = vox.0;
            s.extend_from_slice(&x.0);

            return Some(Signature { salt, s: FieldVector(s) });
        }
        // L was singular → no solution to gauss
    }

    None // Failed 
}

/// Verify a signature against a message and public key
pub fn verify(pk: &PublicKey, message: &[u8], sig: &Signature) -> bool{
    let s = &sig.s;
    // early about if not same length
    if s.0.len() != N {
        return false;
    }

    // Recompute t = Hash(message || salt)
    let t = hash_message(message, &sig.salt, M);

    // Split s into sv and so
    let sv = FieldVector(s.0[..V].to_vec());
    let so = FieldVector(s.0[V..].to_vec());

    for i in 0..M {

        // sv^T P1_i sv
        let p1_sv = pk.p1_matrices[i].clone() * sv.clone();
        let svt_p1_sv = sv.0.iter().zip(p1_sv.0.iter())
            .map(|(a, b)| *a * *b)
            .fold(F16Element::zero(), |acc, x| acc + x);

        // sv^T P2_i so
        let p2_so = pk.p2_matrices[i].clone() * so.clone();
        let svt_p2_so = sv.0.iter().zip(p2_so.0.iter())
            .map(|(a, b)| *a * *b)
            .fold(F16Element::zero(), |acc, x| acc + x);

        // so^T P3_i so
        let p3_so = pk.p3_matrices[i].clone() * so.clone();
        let sot_p3_so = so.0.iter().zip(p3_so.0.iter())
            .map(|(a, b)| *a * *b)
            .fold(F16Element::zero(), |acc, x| acc + x);

        let result = svt_p1_sv + svt_p2_so + sot_p3_so;

        if result != t.0[i] {
            return false;
        }
    }

    true

}