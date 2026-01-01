#[cfg(test)]
mod tests {
    use super::*;
    // This helper simulates the deterministic expected signature computation.
    fn compute_expected_signature(message: &[u8], public_key: &[u8]) -> Vec<u8> {
        let mut sig = vec![0u8; 64];
        for i in 0..64 {
            let m = message.get(i % message.len()).unwrap();
            let p = public_key.get(i % public_key.len()).unwrap();
            sig[i] = m ^ p;
        }
        sig
    }

    #[test]
    fn test_signature_verification() {
        let message = b"Test Message";
        let public_key = vec![1u8; 32];
        // Invalid signature that does not match the computed expected signature.
        let invalid_signature = vec![0u8; 64];
        // Valid signature is computed deterministically.
        let valid_signature = compute_expected_signature(message, &public_key);

        // Instantiate the packet using the vulnerable implementation.
        let vuln_packet = DataPacket::new(message, &invalid_signature, &public_key);
        // In the vulnerable version, verify erroneously returns true.
        // The test should fail when running against vulnerable code.
        assert!(
            !vuln_packet.verify(),
            "Test Failed: Signature verification should fail for invalid signature in secure implementation"
        );

        // Instantiate the packet using the corrected implementation.
        let fix_packet = DataPacket::new(message, &valid_signature, &public_key);
        // Secure verification: valid signature must pass.
        assert!(
            fix_packet.verify(),
            "Test Failed: Signature verification should pass for valid signature in secure implementation"
        );
    }
}