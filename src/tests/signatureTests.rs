#[cfg(test)]
mod tests {
    use crate::keys::keygen;
    use crate::signature::{sign, verify};
    use crate::matrix::FieldVector;
    use crate::field::F16Element;
    use crate::signature::Signature;

    fn setup() -> (crate::keys::PublicKey, crate::keys::SecretKey) {
        keygen()
    }

    #[test]
    fn test_sign_verify_correct() {
        let (pk, sk) = setup();
        let message = b"test message";
        let sig = sign(&sk, message).expect("signing failed");
        assert!(verify(&pk, message, &sig), "should verify");
    }

    #[test]
    fn test_verify_wrong_message() {
        let (pk, sk) = setup();
        let sig = sign(&sk, b"correct message").expect("signing failed");
        assert!(!verify(&pk, b"wrong message", &sig), "should not verify");
    }

    #[test]
    fn test_verify_tampered_signature() {
        let (pk, sk) = setup();
        let message = b"test message";
        let sig = sign(&sk, message).expect("signing failed");

        let mut tampered_s = sig.s.0.clone();
        // flip the last to bits in the 42th element
        tampered_s[42] = F16Element(tampered_s[42].0 ^ 0x3);
        let tampered = Signature { salt: sig.salt, s: FieldVector(tampered_s) };

        assert!(!verify(&pk, message, &tampered), "tampered signature should not verify");
    }

    #[test]
    fn test_verify_tampered_salt() {
        let (pk, sk) = setup();
        let message = b"test message";
        let sig = sign(&sk, message).expect("signing failed");

        let mut tampered_salt = sig.salt;
        tampered_salt[0] ^= 0xFF;
        let tampered = Signature { salt: tampered_salt, s: sig.s.clone() };

        assert!(!verify(&pk, message, &tampered), "tampered salt should not verify");
    }

    #[test]
    fn test_sign_different_messages_differ() {
        let (_pk, sk) = setup();
        let sig1 = sign(&sk, b"message one").expect("signing failed");
        let sig2 = sign(&sk, b"message two").expect("signing failed");
        assert_ne!(sig1.s.0, sig2.s.0, "signatures should differ");
    }
}