mod field;
mod hash;
mod keys;
mod matrix;
mod signature;

use keys::keygen;
use signature::{sign, verify};

fn main() {

    let (pk, sk) = keygen();
 
    let message = b"post-quantum salatmaxxing";
    println!("Message: {}", std::str::from_utf8(message).unwrap());

    let sig = sign(&sk, message).expect("signing failed");
 
    let valid = verify(&pk, message, &sig);
    println!("Signature valid: {valid}");

}
