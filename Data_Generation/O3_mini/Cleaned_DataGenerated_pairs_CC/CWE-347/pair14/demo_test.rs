//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    /// The oracle tests that an invalid message (with an untrusted context) is rejected.
    /// In the fixed version the verification function returns false,
    /// whereas, in the vulnerable version, it would mistakenly return true.
    #[test]
    fn test_oracle() {
        // Create a message with an invalid context but with a signature vector of 64 bytes.
        let msg = SecureMessage::new(vec![1, 2, 3], vec![0u8; 64], "untrusted_domain".to_owned());
        // In a secure implementation, invalid messages must be rejected.
        let result = verify_message(&msg);
        assert!(!result, "Oracle test failed: invalid message was accepted.");
    }
}