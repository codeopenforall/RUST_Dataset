#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn test_signature_verification() {
        let pubkey = Arc::new(vec![1, 2, 3, 4]);
        let validator = Validator { pubkey };
        let message = b"Attack at dawn";
        let invalid_signature = b"fake_signature";
        assert_eq!(
            validator.process_message(message, invalid_signature),
            false,
            "Signature verification should fail for an invalid signature."
        );
        let valid_signature = b"VALID_SIGNATURE";
        assert_eq!(
            validator.process_message(message, valid_signature),
            true,
            "Signature verification should pass for a valid signature."
        );
    }
}
