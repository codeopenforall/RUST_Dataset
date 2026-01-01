#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication() {
        let auth = Authenticator::new();
        let result = auth.verify("alice", "wrong_password");
        let error_message = match result {
            Ok(_) => "Login successful".to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(error_message, "Invalid credentials", "Authentication error should be generic");
    }
}
