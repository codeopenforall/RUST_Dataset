#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signature_verification() {
        let msg = b"Test message".to_vec();
        let mut sign: [u8; 64] = [0u8; 64];
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
        assert!(!result, "Packet verification should fail for an unauthorized algorithm.");
    }
}
