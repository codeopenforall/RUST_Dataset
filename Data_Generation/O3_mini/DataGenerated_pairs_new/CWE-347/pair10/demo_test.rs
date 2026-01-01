#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle is used to validate the contract:
    // For a packet with an unauthorized algorithm ("malformed"), a proper implementation must reject the signature.
    // The vulnerable code erroneously accepts such a packet, while the corrected code returns false.
    #[test]
    fn test_signature_verification() {
        // Build a packet with a "malformed" algorithm.
        let msg = b"Test message".to_vec();
        let mut sign: [u8; 64] = [0u8; 64];
        // Craft the signature so that in the unsafe verifier, every byte of the message cancels out (i.e.  value equals msg cyclically).
        for i in 0..64 {
            sign[i] = msg[i % msg.len()];
        }
        let packet = Packet {
            message: msg,
            signature: sign,
            algorithm: "malformed".to_string(),
        };

        let handler = CryptoHandler;
        let result = handler.verify(&packet);
        // The contract: packets with an unauthorized algorithm must be rejected.
        assert!(!result, "Packet verification should fail for an unauthorized algorithm.");
    }
}