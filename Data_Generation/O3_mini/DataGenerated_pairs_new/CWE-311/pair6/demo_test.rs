#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_transmission() {
        let config = Arc::new(Config::new());
        let transmitted = transmit(&config);
        // In the vulnerable implementation, the transmitted bytes would exactly equal
        // b"sensitive_data", causing the assertion to fail.
        // In the secure implementation, transmitted data is encrypted and must not match the plaintext.
        assert_ne!(transmitted, b"sensitive_data".to_vec(), "Sensitive data should not be transmitted in plaintext");
    }
}