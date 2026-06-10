use crate::keys::{PublicKey, SecretKey};
use crate::matrix::FieldVector;
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
pub fn sign(sk: &SecretKey, message: &[u8]) -> Signature{
    todo!()
}

/// Verify a signature against a message and public key
pub fn verify(pk: &PublicKey, message: &[u8], sig: &Signature) -> bool{
    todo!()
}