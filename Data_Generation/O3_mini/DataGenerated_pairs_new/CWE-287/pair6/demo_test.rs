//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::AuthService;

    #[test]
    fn authentication_oracle() {
        // Initialization common to both implementations.
        let auth = AuthService { valid_token: "secure123".to_string() };
        
        // For a token that should be invalid:
        // In the secure implementation, "test" must not be accepted.
        // In the vulnerable implementation, "test" is erroneously accepted.
        unsafe {
            assert_eq!(
                auth.validate("test"),
                false,
                "Oracle: Token 'test' should be rejected"
            );
        }
        
        // For a valid token, the check should succeed.
        unsafe {
            assert_eq!(
                auth.validate("secure123"),
                true,
                "Oracle: Token 'secure123' should be accepted"
            );
        }
    }
}