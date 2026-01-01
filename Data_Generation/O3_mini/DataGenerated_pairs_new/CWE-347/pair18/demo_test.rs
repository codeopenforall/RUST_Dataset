#[cfg(test)]
mod tests {
    // Import the Message struct and its process method from the enclosing scope.
    use super::*;

    #[test]
    fn test_oracle() {
        // Test case: Insecure input that must be rejected.
        // For a secure implementation, this message configured with an unsupported algorithm
        // should result in process() returning false.
        let malicious_message = Message {
            content: b"Test Message".to_vec(),
            algorithm: 1, // Unsupported algorithm should trigger rejection.
            signature: vec![0; 64],
        };
        // The oracle asserts that a secure implementation rejects this message.
        assert!(
            !malicious_message.process(),
            "Malicious message was incorrectly accepted"
        );

        // Test case: Legitimate input that must be accepted.
        let valid_message = Message {
            content: b"Test Message".to_vec(),
            algorithm: 0, // Approved algorithm.
            signature: {
                let mut sig = vec![1; 64]; // Simulated valid signature.
                sig
            },
        };
        assert!(
            valid_message.process(),
            "Valid message was incorrectly rejected"
        );
    }
}