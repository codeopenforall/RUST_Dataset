#[cfg(test)]
mod tests {
    use super::*;
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
        let invalid_signature = vec![0u8; 64];
        let valid_signature = compute_expected_signature(message, &public_key);
        let vuln_packet = DataPacket::new(message, &invalid_signature, &public_key);
        assert!(
            !vuln_packet.verify(),
            "Test Failed: Signature verification should fail for invalid signature in secure implementation"
        );
        let fix_packet = DataPacket::new(message, &valid_signature, &public_key);
        assert!(
            fix_packet.verify(),
            "Test Failed: Signature verification should pass for valid signature in secure implementation"
        );
    }
}
