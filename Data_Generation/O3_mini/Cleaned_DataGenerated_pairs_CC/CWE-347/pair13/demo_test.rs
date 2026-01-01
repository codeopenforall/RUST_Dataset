//////////////////// oracle_test.rs ////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Test oracle that ensures a forged signature is rejected.
    // In the vulnerable version, the forged signature would be accepted,
    // causing this test to fail. In the fixed version, the forged signature is
    // correctly rejected, and the test passes.
    #[test]
    fn test_signature_verification() {
        // Prepare test data.
        let message = b"Test message".to_vec();
        let pubkey = b"TestPublicKey".to_vec();
        // Forge a signature that is 32 bytes of zeros.
        let forged_signature = vec![0u8; 32];
        let result = concurrent_check(Arc::new(message), Arc::new(forged_signature), Arc::new(pubkey));
        assert!(!result, "Forged signature should not verify");
    }
}