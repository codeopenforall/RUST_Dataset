#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let msg = SecureMessage::new(vec![1, 2, 3], vec![0u8; 64], "untrusted_domain".to_owned());
        let result = verify_message(&msg);
        assert!(!result, "Oracle test failed: invalid message was accepted.");
    }
}
