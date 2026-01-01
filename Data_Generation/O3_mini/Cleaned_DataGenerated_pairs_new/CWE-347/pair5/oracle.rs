#[cfg(test)]
mod tests {
    use super::Verifier;
    #[test]
    fn test_signature_verification() {
        let message = b"Important confidential message";
        let public_key = [0u8; 32];
        let mut signature = [1u8; 65];
        signature[0] = 0xFF;
        let valid = Verifier::check(message, &public_key, &signature);
        assert!(!valid, "Signature verification MUST fail for invalid algorithm identifier");
    }
}
