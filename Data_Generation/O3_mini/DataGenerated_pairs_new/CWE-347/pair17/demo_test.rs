#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        // Construct a packet with a bogus signature that does not match the expected computed hash.
        let packet = DataPacket {
            message: b"Test input".to_vec(),
            // An arbitrary signature that should be rejected by a correct verification routine.
            signature: vec![9, 8, 7, 6, 5, 4, 3, 2],
        };
        let result = verify_logic(&packet);
        // The contract is that invalid signatures must be rejected.
        assert!(!result, "Oracle: Verification should fail for an invalid signature");
    }
}