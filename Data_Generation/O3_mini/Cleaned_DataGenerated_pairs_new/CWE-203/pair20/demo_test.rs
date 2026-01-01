#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authenticator_error_consistency() {
        // Prepare a known set of accounts.
        let accounts = vec![
            Account { username: "alice".to_string(), password: "wonderland".to_string() },
            Account { username: "bob".to_string(), password: "builder".to_string() },
        ];
        let auth = Authenticator::new(accounts);
        
        // Trigger the authentication check for an existing user with a wrong password.
        let error_existing = auth.verify("alice", "wrong").err().expect("Expected error");
        
        // Trigger the authentication check for a non-existent user.
        let error_nonexistent = auth.verify("charlie", "wrong").err().expect("Expected error");
        
        // In the fixed version both error messages are identical.
        // In the vulnerable version, these errors differ, leaking information.
        assert_eq!(error_existing, error_nonexistent, "Error messages leak sensitive information");
    }
}