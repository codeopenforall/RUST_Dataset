#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authenticator_error_consistency() {
        let accounts = vec![
            Account { username: "alice".to_string(), password: "wonderland".to_string() },
            Account { username: "bob".to_string(), password: "builder".to_string() },
        ];
        let auth = Authenticator::new(accounts);
        let error_existing = auth.verify("alice", "wrong").err().expect("Expected error");
        let error_nonexistent = auth.verify("charlie", "wrong").err().expect("Expected error");
        assert_eq!(error_existing, error_nonexistent, "Error messages leak sensitive information");
    }
}
