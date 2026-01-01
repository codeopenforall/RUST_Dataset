#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let malicious_message = Message {
            content: b"Test Message".to_vec(),
            algorithm: 1, 
            signature: vec![0; 64],
        };
        assert!(
            !malicious_message.process(),
            "Malicious message was incorrectly accepted"
        );
        let valid_message = Message {
            content: b"Test Message".to_vec(),
            algorithm: 0, 
            signature: {
                let mut sig = vec![1; 64]; 
                sig
            },
        };
        assert!(
            valid_message.process(),
            "Valid message was incorrectly rejected"
        );
    }
}
