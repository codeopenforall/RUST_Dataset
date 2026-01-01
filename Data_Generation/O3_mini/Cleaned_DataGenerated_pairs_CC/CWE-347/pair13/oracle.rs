#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn test_signature_verification() {
        let message = b"Test message".to_vec();
        let pubkey = b"TestPublicKey".to_vec();
        let forged_signature = vec![0u8; 32];
        let result = concurrent_check(Arc::new(message), Arc::new(forged_signature), Arc::new(pubkey));
        assert!(!result, "Forged signature should not verify");
    }
}
