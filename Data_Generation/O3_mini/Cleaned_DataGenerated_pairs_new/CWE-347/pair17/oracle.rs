#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let packet = DataPacket {
            message: b"Test input".to_vec(),
            signature: vec![9, 8, 7, 6, 5, 4, 3, 2],
        };
        let result = verify_logic(&packet);
        assert!(!result, "Oracle: Verification should fail for an invalid signature");
    }
}
